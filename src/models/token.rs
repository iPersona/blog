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

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::HttpResponse;
use futures::future::{ok, Either, FutureResult};
use futures::Poll;
use log::{debug, error, info};

macro_rules! token_check_error {
    ($req:expr) => {
        ok($req.into_response(HttpResponse::Forbidden().finish().into_body()))
    };
}

macro_rules! token_expired_error {
    ($req:expr) => {
        ok($req.into_response(HttpResponse::Gone().finish().into_body()))
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user id
    user_id: String,
    // user type
    user_type: i16,
    // user name
    user_name: String,
    // nick name
    user_nickname: String,
    // email
    email: String,
    // is admin
    is_admin: bool,
}

impl Token {
    pub fn new(user: &UserInfo) -> Self {
        Token {
            iss: "https://www.coimioc.com".to_string(),
            sub: "Register".to_string(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
            user_id: user.id.to_hyphenated().to_string(),
            user_type: user.groups,
            user_name: user.account.clone(),
            user_nickname: user.nickname.clone(),
            email: user.email.clone(),
            is_admin: user.is_admin(),
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
    // type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // TODO: 需要重写
        let path = req.path();
        info!("path: {:?}", path);
        info!("method: {:?}", req.method());

        let token = req.headers().get("Authorization");
        match token {
            Some(t) => {
                let t = Token::decode(t.to_str().unwrap());
                let result = match t {
                    Ok(t) => Permission::new(path, Some(&t)).check(Some(&t)),
                    Err(e) => match e.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                            // 重新登录
                            Err("TokenExpired".to_string())
                        }
                        _ => Permission::new(path, None).check(None),
                    },
                };
                match result {
                    Ok(is_ok) => {
                        if is_ok {
                            Either::A(self.service.call(req))
                        } else {
                            Either::B(token_check_error!(req))
                        }
                    }
                    Err(e) => {
                        if e.eq("TokenExpired") {
                            Either::B(token_expired_error!(req))
                        } else {
                            Either::B(token_check_error!(req))
                        }
                    }
                }
            }
            None => {
                // TODO: 跳转到登录界面
                let result = Permission::new(path, None).check(None);
                match result {
                    Ok(is_ok) => {
                        if is_ok {
                            Either::A(self.service.call(req))
                        } else {
                            Either::B(token_check_error!(req))
                        }
                    }
                    Err(_e) => Either::B(token_check_error!(req)),
                }
            }
        }
        //
        //        let ctx = get_identity_and_web_context(&mut req);
        //        req.extensions_mut().insert(ctx);
        //
        //        // info!("middleware-finish");
        //        // if let Ok(Some(result)) = req.get_session().get::<String>("token") {
        //        //     info!("session value new: {:?}", result);
        //        // } else {
        //        //     info!("get session value new failed");
        //        // }
        //        Box::new(self.service.call(req).map(move |res| res))
    }
}

pub enum UserUrl {
    ChangePassword,
    View,
    SignOut,
    Edit,
    NewComment,
    DeleteComment,
}

impl UserUrl {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "/user/change_pwd" => Some(Self::ChangePassword),
            "/user/view" => Some(Self::View),
            "/user/sign_out" => Some(Self::SignOut),
            "/user/edit" => Some(Self::Edit),
            "/user/new_comment" => Some(Self::NewComment),
            "/user/delete_comment" => Some(Self::DeleteComment),
            _ => None
        }
    }
}

pub enum VisitorUrl {
    ListAllArticles,
    ViewComment,
    ViewArticle,
    Login,
    NewUser,
    UserExist,
    GetArticleCount,
    GetTagsWithCount,
    GetTagsWithoutCount,
}

impl VisitorUrl {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "/articles" => Some(Self::ListAllArticles),
            "/article/view_comment" => Some(Self::ViewComment),
            "/article/view" => Some(Self::ViewArticle),
            "/user/login" => Some(Self::Login),
            "/user/new" => Some(Self::NewUser),
            "/user/exist" => Some(Self::UserExist),
            "/article/count" => Some(Self::GetArticleCount),
            "/tag/view/count" => Some(Self::GetTagsWithCount),
            "/tag/view" => Some(Self::GetTagsWithoutCount),
            _ => None,
        }
    }
}

pub enum Url {
    UserUrl(UserUrl),
    VisitorUrl(VisitorUrl)
}

impl Url {
    pub fn from_str(s: &str) -> Option<Url> {
        let user_url = UserUrl::from_str(s);
        if user_url.is_some() {
            return Some(Self::UserUrl(user_url.unwrap()));
        }

        let visitor_url = VisitorUrl::from_str(s);
        if visitor_url.is_some() {
            return Some(Self::VisitorUrl(visitor_url.unwrap()));
        }

        None
    }
}

pub struct Permission {
    pub url: String,
    pub permission_type: PermissionType,
}

impl Permission {
    pub fn new(url: &str, token: Option<&Token>) -> Self {
        match token {
            Some(t) => Permission {
                url: url.to_string(),
                permission_type: if t.user_type == 0 {
                    PermissionType::Admin
                } else {
                    PermissionType::Registered
                },
            },
            None => Permission {
                url: url.to_string(),
                permission_type: PermissionType::Visitor,
            },
        }
    }

    pub fn check(&self, token: Option<&Token>) -> std::result::Result<bool, String> {
        // TODO: 检查jwt是否已经超时，若超时，让客户端重新登录，以获取token
        match token {
            Some(t) => {
                if Utc::now().timestamp() > t.exp {
                    // token expirated
                    return Err("JWT expired!".to_string());
                }
                // 检查当前用户权限是否足够访问资源
                match self.permission_type {
                    PermissionType::Admin => Ok(true),
                    PermissionType::Registered => Ok(self.is_user_permission()),
                    PermissionType::Visitor => Ok(self.is_visitor_permission()),
                }
            }
            None => Ok(self.is_visitor_permission()),
        }
    }

    fn is_user_permission(&self) -> bool {
        let user_url = UserUrl::from_str(self.url.as_str());
        let visitor_url = VisitorUrl::from_str(self.url.as_str());
        user_url.is_some() || visitor_url.is_some()

        // let user_resources = [
        //     "/user/change_pwd",
        //     "/user/view",
        //     "/user/sign_out",
        //     "/user/edit",
        //     "/user/new_comment",
        //     "/user/delete_comment",
        // ];
        // user_resources.contains(&self.url.as_str())
    }

    fn is_visitor_permission(&self) -> bool {
        use crate::util::path::{path_components_num, path_without_last_component};

        let p = if path_components_num(self.url.as_str()) >= 4 {
            path_without_last_component(self.url.as_str())
        } else {
            self.url.as_str().to_string()
        };
        debug!("p: {:?}", p);
        let visitor_url = VisitorUrl::from_str(&p.as_str());
        visitor_url.is_some()


        // let visitor_resources = [
        //     "/articles",
        //     "/article/view_comment",
        //     "/article/view",
        //     "/user/login",
        //     "/user/new",
        //     "/user/exist",
        //     "/article/count",
        // ];

        // visitor_resources.contains(&p.as_str())
    }
}

pub enum PermissionType {
    Admin,
    Registered,
    Visitor,
}
