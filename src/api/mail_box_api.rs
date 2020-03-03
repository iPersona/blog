use crate::util::errors::ErrorCode;
use crate::{
    models::{
        mailbox::mail_box::{CommentNotify, CommentNotifyParam},
        token::TokenExtension,
    },
    AppState,
};
use actix_web::{
    web::{Data, Query},
    Error, HttpRequest, HttpResponse,
};
use futures::Future;

pub struct MailboxApi;

impl MailboxApi {
    pub fn user_comment_notifies(
        state: Data<AppState>,
        req: HttpRequest,
        params: Query<CommentNotifyParam>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("user_comment_notifies");
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
                    Some(_) => {
                        let conn = &state.db.connection();
                        let user_id = params.into_inner().user_id;
                        let res = CommentNotify::user_notifies(user_id, conn);
                        match res {
                            Ok(data) => api_resp_data!(data),
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
}
