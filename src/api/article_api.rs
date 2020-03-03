use actix_web::web::{Data, Form, Path, Query};
use actix_web::{Error, HttpRequest, HttpResponse};
use futures::future::Future;
use futures::stream::Stream;
use uuid::Uuid;

use crate::models::articles::{
    AdminViewRawArticle, ArticleSummary, ModifyPublish, QuerySlice, ViewArticle,
};
use crate::models::token::TokenExtension;
use crate::util::errors::ErrorCode;
use crate::{AppState, ArticleList, ArticlesWithTag, EditArticle, NewArticle};
use actix_web::web;

pub struct ArticleApi;

impl ArticleApi {
    pub fn create_article(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("create article");
        extract_form_data!(NewArticle, req, body, &state)
    }

    pub fn delete_article(
        state: Data<AppState>,
        req: HttpRequest,
        article_id: Path<Uuid>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("delete article");
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let res = ArticlesWithTag::delete_with_id(state.get_ref(), article_id.into_inner());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(e) => api_resp_err!(&e[..]),
        }
    }

    pub fn admin_view_article(
        state: Data<AppState>,
        _req: HttpRequest,
        query: Query<ViewArticle>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("admin_view_article");
        debug!("id={:?}", query.id);
        let res = ArticlesWithTag::query_without_article(state.get_ref(), query.id, true);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(e) => api_resp_err!(format!("query_without_article failed: {:?}", e)),
        }
    }

    pub fn admin_view_raw_article(
        state: Data<AppState>,
        _req: HttpRequest,
        query: Query<AdminViewRawArticle>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("admin_view_raw_article");
        info!("params:{:?}", query.id.as_str());
        let res = ArticlesWithTag::query_raw_article(
            state.get_ref(),
            Uuid::parse_str(query.id.as_str()).unwrap(),
        );
        match res {
            Ok(data) => api_resp_data!(data),
            Err(e) => api_resp_err!(format!("query_raw_article failed: {:?}", e)),
        }
    }

    pub fn admin_list_all_article(
        state: Data<AppState>,
        _req: HttpRequest,
        query: Query<QuerySlice>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("admin_list_all_article");
        let res = ArticleList::query_list_article(&state, query.limit, query.offset, true);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("admin_list_all_article failed!"),
        }
    }

    pub fn edit_article(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("edit_article");
        extract_form_data!(EditArticle, req, body, &state)
    }

    pub fn update_publish(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<ModifyPublish>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("update_publish");
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return api_resp_err_with_code!(
                ErrorCode::PermissionDenied,
                "Permission denied, this API is for administrator only".to_string()
            );
        }

        let res = ArticlesWithTag::publish_article(state.get_ref(), &params);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("update_publish failed!"),
        }
    }

    fn view_article(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Path<Uuid>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("view_article: {:?}", &params);
        match ArticlesWithTag::query_article(&state, params.into_inner(), false) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    pub fn list_all_article(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Query<QuerySlice>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("list_all_article");
        let conn = &state.db.connection();
        let res = ArticleSummary::list_articles(conn, params.limit, params.offset, false);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("list_all_article failed!"),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/article")
                .route(web::post().to_async(Self::create_article)) // create
                .route(web::put().to_async(Self::edit_article)) // update
                .route(web::patch().to_async(Self::update_publish)), // publish
        )
        .service(
            web::resource("/article/{article_id}")
                .route(web::delete().to_async(Self::delete_article)) // delete
                .route(web::get().to_async(Self::view_article)), // get article
        )
        .service(
            web::resource("/articles").route(web::get().to_async(Self::list_all_article)), // list articles
        );
    }
}
