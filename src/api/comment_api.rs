use crate::api::ApiResult;
use crate::models::{
    comment::{CommentLocation, CommentLocationArgs, CommentQueryOption},
    token::TokenExtension,
};
use crate::util::errors::ErrorCode;
use crate::util::result::InternalStdResult;
use crate::{AppState, Comments, DeleteComment, NewComments, SubComment};
use actix_web::web::{Data, Path, Query};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::PgConnection;
use uuid::Uuid;

pub struct CommentApi;

impl CommentApi {
    async fn new_comment(
        state: Data<AppState>,
        req: HttpRequest,
        mut body: web::Payload,
    ) -> Result<HttpResponse, Error> {
        debug!("new_comment");
        let res = extract_data!(body, NewComments);
        match res {
            Ok(data) => match data.execute(req, &state).await {
                Ok(_) => api_resp_ok!(),
                Err(e) => api_resp_err_with_code!(e.code, e.detail),
            },
            Err(e) => api_resp_err_with_code!(e.code, e.detail),
        }
    }

    async fn delete_comment(
        state: Data<AppState>,
        req: HttpRequest,
        mut body: web::Payload,
    ) -> Result<HttpResponse, Error> {
        debug!("delete_comment");
        let res = extract_data!(body, DeleteComment);
        match res {
            Ok(data) => match data.execute(req, &state).await {
                Ok(_) => api_resp_ok!(),
                Err(e) => api_resp_err_with_code!(e.code, e.detail),
            },
            Err(e) => api_resp_err_with_code!(e.code, e.detail),
        }
    }

    async fn list_comments(
        state: Data<AppState>,
        _req: HttpRequest,
        article_id: Path<Uuid>,
        params: Query<CommentQueryOption>,
    ) -> Result<HttpResponse, Error> {
        info!("list_comments");
        let conn = state.get_ref().db.connection();
        let is_top_comment = params.parent_comment.is_none();
        let res = if is_top_comment {
            // request top level comments
            Self::top_level_comments(&conn, article_id.into_inner(), &params)
        } else {
            // request sub comments
            Self::sub_comments(&conn, article_id.into_inner(), &params)
        };
        match res {
            Ok(data) => api_resp!(data),
            Err(e) => api_resp_err_with_code!(e.code, e.detail),
        }
    }

    fn top_level_comments(
        conn: &PgConnection,
        article_id: Uuid,
        params: &Query<CommentQueryOption>,
    ) -> InternalStdResult<ApiResult> {
        let res = match params.user_id {
            Some(uid) => {
                Comments::user_comments(conn, params.limit, params.offset, article_id, uid.clone())
            }
            None => Comments::first_class_comments(conn, params.limit, params.offset, article_id),
        };
        match res {
            Ok(data) => Ok(ApiResult::from_data(data)),
            Err(err) => Err(crate::util::errors::Error {
                code: ErrorCode::DbError,
                detail: err.clone(),
            }),
        }
    }

    fn sub_comments(
        conn: &PgConnection,
        article_id: Uuid,
        params: &Query<CommentQueryOption>,
    ) -> InternalStdResult<ApiResult> {
        let res = SubComment::comments(
            conn,
            params.limit,
            params.offset,
            article_id,
            params.parent_comment.unwrap(),
        );
        match res {
            Ok(data) => Ok(ApiResult::from_data(data)),
            Err(err) => Err(crate::util::errors::Error {
                code: ErrorCode::DbError,
                detail: err.clone(),
            }),
        }
    }

    async fn locate_comment(
        state: Data<AppState>,
        req: HttpRequest,
        params: Query<CommentLocationArgs>,
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
                let args = params.into_inner();
                if !t.is_user(args.user_id) {
                    return api_resp_err_with_code!(
                        ErrorCode::PermissionDenied,
                        "User not match, operation denied!"
                    );
                }

                let conn = &state.db.connection();
                let res = CommentLocation::locate(conn, &args);
                match res {
                    Ok(data) => api_resp!(ApiResult::from_raw_data(data.data)),
                    Err(e) => api_resp_err_with_code!(e.code, e.detail),
                }
            }
            None => {
                return api_resp_err_with_code!(
                    ErrorCode::PermissionDenied,
                    "This API is for login user only!"
                );
            }
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/comment").route(web::post().to(Self::new_comment)), // create comment
        )
        .service(
            web::resource("/comment/{comment_id}").route(web::delete().to(Self::delete_comment)), // delete comment
        )
        .service(web::resource("/comments/{article_id}").route(web::get().to(Self::list_comments)))
        .service(web::resource("/location/comment").route(web::get().to(Self::locate_comment)));
    }
}
