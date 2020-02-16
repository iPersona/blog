use crate::models::articles::QuerySlice;
use crate::models::comment::CommentQueryOption;
use crate::{AppState, Comments, DeleteComment, NewComments};
use actix_web::web::{Data, Path, Query};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::stream::Stream;
use futures::Future;
use log::{debug, info};
use uuid::Uuid;

pub struct CommentApi;

impl CommentApi {
    fn new_comment(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("new_comment");
        extract_form_data!(NewComments, req, body, &state)
    }

    fn delete_comment(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("delete_comment");
        extract_form_data!(DeleteComment, req, body, &state)
    }

    fn list_comments(
        state: Data<AppState>,
        _req: HttpRequest,
        article_id: Path<Uuid>,
        params: Query<CommentQueryOption>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("list_comments");
        let pg_pool = state.get_ref().db.connection();
        let res = match params.user_id {
            Some(uid) => Comments::query_with_user(
                &pg_pool,
                params.limit,
                params.offset,
                article_id.into_inner(),
                uid.clone(),
            ),
            None => Comments::query(
                &pg_pool,
                params.limit,
                params.offset,
                article_id.into_inner(),
            ),
        };
        match res {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/comment").route(web::post().to_async(Self::new_comment)), // create comment
        )
        .service(
            web::resource("/comment/{comment_id}")
                .route(web::delete().to_async(Self::delete_comment)), // delete comment
        )
        .service(
            web::resource("/comments/{article_id}").route(web::get().to_async(Self::list_comments)),
        );
    }
}
