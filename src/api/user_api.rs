use crate::api::recaptcha_api::verify_recaptcha;
use crate::models::token::{Token, TokenExtension};
use crate::models::user::{CheckUser, LoginUser};
use crate::util::errors::ErrorCode;
use crate::{AppState, ChangePassword, EditUser, RegisteredUser};
use actix_web::web::{Data, Form};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::stream::Stream;
use futures::Future;
use log::debug;

pub struct UserApi;

impl UserApi {
    fn change_pwd(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<ChangePassword>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
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

    fn edit(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("edit_user");
        extract_form_data!(EditUser, req, body, &state)
    }

    fn sign_out(
        _state: Data<AppState>,
        _req: HttpRequest,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        api_resp_ok!()
    }

    fn create_user(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<RegisteredUser>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        match params.into_inner().insert(&state) {
            Ok(_) => api_resp_ok!(),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn is_user_exist(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<CheckUser>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let pg_pool = state.db.connection();
        let exist = params.into_inner().is_user_exist(&pg_pool);
        api_resp_data!(exist)
    }

    fn login(
        state: Data<AppState>,
        mut _req: HttpRequest,
        params: Form<LoginUser>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let params = &params.into_inner();

        // verify reCAPTCHA
        let is_ok = verify_recaptcha(params.token().as_str());
        if !is_ok {
            return api_resp_err!("robot detected!");
        }

        let is_remember = params.get_remember();
        let max_age: Option<i64> = if is_remember { Some(24 * 90) } else { None };

        let pg_pool = state.db.connection();
        match params.verification(&pg_pool, &max_age) {
            Ok(user_info) => {
                let token = Token::new(&user_info);
                match token.encode() {
                    Ok(v) => api_resp_data!(v),
                    Err(e) => api_resp_err!(format!("{:?}", e)),
                }
            }
            Err(err) => api_resp_err!(&*err),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/user/password").route(web::patch().to_async(Self::change_pwd)))
            .service(
                web::resource("/user")
                    .route(web::put().to_async(Self::edit))
                    .route(web::post().to_async(Self::create_user))
                    .route(web::get().to_async(Self::is_user_exist)),
            )
            .service(web::resource("/login").route(web::post().to_async(Self::login)))
            .service(web::resource("/logout").route(web::get().to_async(Self::sign_out)));
    }
}
