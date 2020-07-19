use crate::api::ApiResult;
use crate::util::errors::ErrorCode;
use crate::{
    models::{mailbox::mail_box::CommentNotify, token::TokenExtension},
    AppState,
};
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};

pub struct MailboxApi;

impl MailboxApi {
    pub async fn comment_notifies(
        state: Data<AppState>,
        req: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        debug!("comment_notifies");
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

                // get comment notifies of user
                match &t.user_info {
                    Some(user) => {
                        let conn = &state.db.connection();
                        let res = CommentNotify::user_notifies(user.id, conn);
                        match res {
                            Ok(data) => api_resp!(ApiResult::from_data(data)),
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
        cfg.service(
            web::resource("/notification/comment").route(web::get().to(Self::comment_notifies)), // list comment notifications
        );
    }
}
