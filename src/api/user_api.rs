use crate::api::recaptcha_api::verify_recaptcha;
use crate::models::token::{Token, TokenExtension};
use crate::models::user::{
    disabled_cookie, user_data as gen_user_data, verify_data, CheckUser, LoginUser, Users, Verify,
};
use crate::util::email::SignUpVerify;
use crate::util::errors::ErrorCode;
use crate::{AppState, ChangePassword, EditUser, RegisteredUser};
use actix_web::web::{Data, Form};
use actix_web::{web, Error, HttpRequest, HttpResponse};

pub struct UserApi;

impl UserApi {
    async fn change_pwd(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<ChangePassword>,
    ) -> Result<HttpResponse, Error> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is allowed
                if !t.is_login() {
                    return api_resp_err_with_code!(
                        ErrorCode::PermissionDenied,
                        "please login first"
                    );
                }

                // Modify password
                match &t.user_info {
                    Some(u) => {
                        let pg_pool = &state.db.connection();
                        match params.into_inner().change_password(u, pg_pool) {
                            Ok(data) => api_resp_data!(data),
                            Err(err) => api_resp_err!(&*err),
                        }
                    }
                    None => {
                        api_resp_err_with_code!(ErrorCode::PermissionDenied, "please login first")
                    }
                }
            }
            None => api_resp_err!(format!("failed to get user info from token")),
        }
    }

    async fn edit(
        state: Data<AppState>,
        req: HttpRequest,
        mut body: web::Payload,
    ) -> Result<HttpResponse, Error> {
        debug!("edit_user");
        let res = extract_data!(body, EditUser);
        match res {
            Ok(data) => match data.execute(req, &state).await {
                Ok(_) => api_resp_ok!(),
                Err(e) => api_resp_err_with_code!(e.code, e.detail),
            },
            Err(e) => api_resp_err_with_code!(e.code, e.detail),
        }
    }

    async fn sign_out(_state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is allowed
                if !t.is_login() {
                    return api_resp_err_with_code!(
                        ErrorCode::PermissionDenied,
                        "please login first"
                    );
                }

                let c = disabled_cookie();
                api_resp_ok!(c)
            }
            None => api_resp_err!(format!("failed to get user info from token")),
        }
    }

    async fn create_user(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<RegisteredUser>,
    ) -> Result<HttpResponse, Error> {
        let user_info = params.into_inner();
        // parse user info
        let verify = SignUpVerify {
            email: user_info.email.clone(),
            user_name: user_info.account.clone(),
        };

        // insert user into database
        let user: Users;
        match user_info.insert(&state) {
            Ok(u) => user = u,
            Err(err) => return api_resp_err!(&*err),
        }

        // generate token
        let token = Token::from_user(user).encode();
        match token {
            Ok(t) => {
                if verify.send_verify_mail(t.as_str()) {
                    return api_resp_ok!();
                }
                return api_resp_err!("send verify email failed!");
            }
            Err(e) => {
                return api_resp_err!(format!("generate token failed: {:?}", e));
            }
        }
    }

    async fn is_user_exist(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<CheckUser>,
    ) -> Result<HttpResponse, Error> {
        let pg_pool = state.db.connection();
        let exist = params.into_inner().is_user_exist(&pg_pool);
        api_resp_data!(exist)
    }

    async fn login(
        state: Data<AppState>,
        mut _req: HttpRequest,
        params: Form<LoginUser>,
    ) -> Result<HttpResponse, Error> {
        let params = &params.into_inner();

        // verify reCAPTCHA
        let is_ok = verify_recaptcha(params.token().as_str()).await;
        if !is_ok {
            return api_resp_err!("robot detected!");
        }

        let is_remember = params.get_remember();
        let max_age: Option<i64> = if is_remember { Some(24 * 90) } else { None };
        let conn = &state.db.connection();

        match params.verification(conn, &max_age) {
            // generate login data
            Ok(user_info) => match gen_user_data(conn, &user_info) {
                Ok(v) => {
                    let token = Token::new(&user_info, true);
                    let c = token.into_cookie().unwrap();
                    api_resp_data!(v, c)
                }
                Err(e) => api_resp_err_with_code!(e.code, e.detail),
            },
            Err(err) => api_resp_err_with_code!(err.code, err.detail),
        }
    }

    pub async fn verify(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<Verify>,
    ) -> Result<HttpResponse, Error> {
        debug!("verify token");

        // decode token
        let token = Token::decode(params.into_inner().token.as_str());
        match token {
            Ok(t) => {
                // verify token
                let conn = &state.db.connection();
                let res = SignUpVerify::verify_token(&t, conn);
                match res {
                    Ok(_) => {
                        // activate account
                        if Users::active_account(conn, &t.user_id().unwrap()) {
                            // return activated token
                            let mut new_token = t.clone();
                            new_token.active();
                            match verify_data(conn, new_token) {
                                Ok(t) => api_resp_data!(t),
                                Err(e) => api_resp_err_with_code!(
                                    ErrorCode::TokenError,
                                    format!("Failed to encode token: {:?}!", e)
                                ),
                            }
                        } else {
                            api_resp_err_with_code!(
                                ErrorCode::Unknown,
                                "Unable to activate account!"
                            )
                        }
                    }
                    Err(_) => api_resp_err_with_code!(
                        ErrorCode::InvalidToken,
                        "Token verification failed!"
                    ),
                }
            }
            Err(_) => api_resp_err_with_code!(ErrorCode::InvalidToken, "Decode token failed!"),
        }
    }

    async fn user_data(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is allowed
                if !t.is_login() {
                    return api_resp_err_with_code!(
                        ErrorCode::PermissionDenied,
                        "please login first"
                    );
                }

                match &t.user_info {
                    Some(u) => {
                        let conn = &state.db.connection();
                        match gen_user_data(conn, u) {
                            Ok(v) => api_resp_data!(v),
                            Err(e) => api_resp_err_with_code!(e.code, e.detail),
                        }
                    }
                    None => {
                        api_resp_err_with_code!(ErrorCode::PermissionDenied, "please login first")
                    }
                }
            }
            None => api_resp_err!(format!("failed to get user info from token")),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/user/password").route(web::patch().to(Self::change_pwd)))
            .service(
                web::resource("/user")
                    .route(web::put().to(Self::edit))
                    .route(web::post().to(Self::create_user))
                    .route(web::get().to(Self::is_user_exist)),
            )
            .service(web::resource("/verify").route(web::post().to(Self::verify)))
            .service(web::resource("/login").route(web::post().to(Self::login)))
            .service(web::resource("/logout").route(web::post().to(Self::sign_out)))
            .service(web::resource("/user/data").route(web::get().to(Self::user_data)));
    }
}
