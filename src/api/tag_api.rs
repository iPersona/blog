use crate::models::articles::{ArticleNumWithTag, ArticleSummary, QuerySlice};
use crate::models::tag::{DeleteTag, TagsData};
use crate::models::token::TokenExtension;
use crate::models::user::UserType;
use crate::util::errors::ErrorCode;
use crate::{AppState, NewTag, TagCount, Tags};
use actix_web::web::{Data, Path, Query};
use actix_web::{web, web::Form, Error, HttpRequest, HttpResponse};
use futures::{Future, Stream};
use log::{debug, info};
use uuid::Uuid;

pub struct TagApi;

impl TagApi {
    pub fn create_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<NewTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let res = params.into_inner().insert(state.get_ref());
        match res {
            true => api_resp_ok!(),
            false => api_resp_err!("create_tag failed!"),
        }
    }

    pub fn delete_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<DeleteTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let res = Tags::delete_tag(state.get_ref(), params.into_inner().id);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("delete_tag failed!"),
        }
    }

    pub fn get_tag_with_count(
        state: Data<AppState>,
        _req: HttpRequest,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = TagCount::view_tag_count(&state.db.connection());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("view_tag_with_count failed!"),
        }
    }

    pub fn get_tag_without_count(
        state: Data<AppState>,
        _req: HttpRequest,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = Tags::view_list_tag(state.get_ref());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("get all tag failed!"),
        }
    }

    pub fn edit_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<Tags>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let res = params.into_inner().edit_tag(state.get_ref());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("edit_tag failed!"),
        }
    }

    pub fn update_tags(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("update_tags");
        extract_form_data!(TagsData, req, body, &state)
    }

    fn list_all_article_filter_by_tag(
        state: Data<AppState>,
        req: HttpRequest,
        tag_id: Path<Uuid>,
        params: Query<QuerySlice>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("list_all_article_filter_by_tag");
        let token_ext = TokenExtension::from_request(&req);
        let is_admin = match token_ext {
            Some(t) => t.user_type == UserType::Admin,
            None => false,
        };
        let conn = &state.get_ref().db.connection();
        match ArticleSummary::list_articles_with_tag(
            conn,
            tag_id.into_inner(),
            params.limit,
            params.offset,
            is_admin,
        ) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn get_article_number_filter_by_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Query<ArticleNumWithTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let token_ext = TokenExtension::from_request(&req);
        let is_admin = match token_ext {
            Some(t) => t.user_type == UserType::Admin,
            None => false,
        };
        let count = ArticleSummary::query_article_numbers_with_tag(&state, params.tag_id, is_admin);
        match count {
            Ok(n) => {
                debug!("article count: {:?}", n);
                api_resp_data!(n)
            }
            Err(e) => api_resp_err!(format!("{:?}", e)),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/tag")
                .route(web::post().to_async(Self::create_tag)) // create
                .route(web::get().to_async(Self::get_tag_without_count)) // get article
                .route(web::delete().to_async(Self::delete_tag)) // delete
                .route(web::patch().to_async(Self::edit_tag)),
        )
        .service(
            web::resource("/tag/{tag_id}/articles")
                .route(web::get().to_async(Self::list_all_article_filter_by_tag)),
        )
        .service(
            web::resource("/tag/{tag_id}/articles/count")
                .route(web::get().to_async(Self::get_article_number_filter_by_tag)),
        )
        .service(
            web::resource("/tags").route(web::put().to_async(Self::update_tags)), // update
        )
        .service(
            web::resource("/tags/articles/count")
                .route(web::get().to_async(Self::get_tag_with_count)),
        );
    }
}
