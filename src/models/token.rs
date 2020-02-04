// How to generate JWT RS256 key
// ssh-keygen -t rsa -b 4096 -m PEM -f jwtRS256.key
// # Don't add passphrase
// openssl rsa -in jwtRS256.key -pubout -outform PEM -out jwtRS256.key.pub
// cat jwtRS256.key
// cat jwtRS256.key.pub

// TODO: HTTP 401 Unauthorized响应。
// 1、安全性问题，不使用https，其他认证方式也存在文章说的问题；
// 2、jwt主动过期问题，完全可以实现，使用黑名单即可；分成两点，客户端要求失效，服务端记录token到黑名单；用户重置密码，服务端记录uid-time键值对，在此之前的token全部失效；
// 3、jwt续签问题，一种解决方式是jwt中存储过期时间，服务端设置刷新时间，请求是判断是否在过期时间或刷新时间，在刷新时间内进行token刷新，失效token记入黑名单；而黑名单过大问题，可以采用记录UID-刷新时间方式解决，判断jwt签发时间，jwt签发时间小于UID-刷新时间的记为失效。
// 不过，把jwt变成有状态替代session确实没啥必要。最后，写文章就写文章，标题党没啥意思。

use crate::models::UserInfo;
use crate::util::env::Env;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Result, Algorithm, Header, Validation};

use crate::models::user::{UserType, Users};
use crate::util::errors::{Error, ErrorCode};
use crate::util::result::InternalStdResult;
use actix_http::httpmessage::HttpMessage;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use log::error;
use typename::TypeName;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, TypeName, Clone)]
pub struct Token {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    //issued at
    pub iat: i64,
    // expiry
    pub exp: i64,
    // user id
    pub user_id: String,
    // user type
    pub user_type: i16,
    // user name
    pub user_name: String,
    // nick name
    pub user_nickname: String,
    // user create time
    pub user_create_time: NaiveDateTime,
    // user signature
    #[serde(skip_serializing_if = "Option::is_none")]
    // remove this field if the value is none. see https://stackoverflow.com/a/53900684
    pub user_sign: Option<String>,
    // email
    pub email: String,
    // is admin
    pub is_admin: bool,
    // is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl Token {
    pub fn new(user: &UserInfo, is_active: bool) -> Self {
        Token {
            iss: Env::get().domain,
            sub: "Register".to_string(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::hours(Env::get().login_token_expired)).timestamp(),
            user_id: user.id.to_hyphenated().to_string(),
            user_type: user.groups,
            user_name: user.account.clone(),
            user_nickname: user.nickname.clone(),
            user_create_time: user.create_time.clone(),
            user_sign: match &user.say {
                Some(v) => Some(v.clone()),
                None => None,
            },
            email: user.email.clone(),
            is_admin: user.is_admin(),
            is_active: if is_active { None } else { Some(false) },
        }
    }

    pub fn from_user(user: Users) -> Token {
        let now = Utc::now();
        Token {
            iss: Env::get().domain,
            sub: "Register".to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(Env::get().verify_token_expired)).timestamp(),
            user_id: user.id.to_hyphenated().to_string(),
            user_type: 1, // 0: admin, other: common user
            user_name: user.account.clone(),
            user_nickname: user.nickname.clone(),
            user_create_time: user.create_time,
            user_sign: user.say,
            email: user.email.clone(),
            is_admin: false,
            is_active: Some(false),
        }
    }

    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: Uuid::parse_str(self.user_id.as_str()).unwrap(),
            account: self.user_name.clone(),
            nickname: self.user_nickname.clone(),
            groups: if self.is_admin() { 0 } else { 1 },
            say: match &self.user_sign {
                Some(v) => Some(v.clone()),
                None => None,
            },
            email: self.email.clone(),
            create_time: self.user_create_time.clone(),
            github: None,
        }
    }

    pub fn decode(t: &str) -> Result<Self> {
        let data = decode::<Self>(
            t,
            Env::get().token_secret.as_bytes(),
            &Validation::default(),
        );
        match data {
            Ok(v) => Ok(v.claims),
            Err(e) => {
                error!("decode token err: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn encode(&self) -> Result<String> {
        let mut header = Header::default();
        header.alg = Algorithm::HS256;
        encode(&header, &self, Env::get().token_secret.as_bytes())
    }

    // pub fn to_base64(encoded: &str) -> String {
    //     base64::encode(encoded.as_bytes())
    // }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    pub fn user_id(&self) -> InternalStdResult<Uuid> {
        match Uuid::parse_str(self.user_id.as_str()) {
            Ok(id) => Ok(id),
            Err(e) => Err(Error {
                code: ErrorCode::ParseError,
                detail: format!("parse uuid from token failed: {:?}", e),
            }),
        }
    }

    /// active account
    pub fn active(&mut self) {
        self.is_active = None;
    }
}

pub struct PermissionControl;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for PermissionControl
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Transform = TokenMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TokenMiddleware { service })
    }
}

pub struct TokenMiddleware<S> {
    service: S,
}

impl<S, B> Service for TokenMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        match token {
            // Registered user or admin
            Some(t) => {
                let t = Token::decode(t.to_str().unwrap());
                match t {
                    Ok(t) => {
                        let user_info = t.to_user_info();
                        // Insert token extension for handler usage
                        req.extensions_mut().insert(TokenExtension {
                            user_info: Some(user_info),
                            user_type: UserType::from_token(Some(&t)),
                            is_active: t.is_active.is_none(),
                        });
                        Either::A(self.service.call(req))
                    }
                    Err(e) => match e.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                            // Need to login again
                            Either::B(middleware_resp_err!(req, crate::util::errors::ErrorCode::TokenExpired, "token expired!"))
                        }
                        _ => Either::B(middleware_resp_err!(req, crate::util::errors::ErrorCode::InvalidToken, "invalid token!")), // Invalid token data
                    },
                }
            }
            // Visitor
            None => {
                req.extensions_mut().insert(TokenExtension {
                    user_info: None,
                    user_type: UserType::Visitor,
                    is_active: false,
                });
                Either::A(self.service.call(req))
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenExtension {
    pub user_info: Option<UserInfo>,
    pub user_type: UserType,
    pub is_active: bool,
}

impl TokenExtension {
    pub fn from_request(req: &HttpRequest) -> Option<Self> {
        let ext = req.extensions();
        let token_ext = ext.get::<TokenExtension>();
        match token_ext {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    pub fn is_login(&self) -> bool {
        self.is_active && self.user_type != UserType::Visitor
    }

    pub fn is_admin(req: &HttpRequest) -> bool {
        let token_ext = Self::from_request(req);
        match token_ext {
            Some(t) => t.user_type == UserType::Admin,
            None => false,
        }
    }
}
