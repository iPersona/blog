use actix_web::web::{Data, Form, Query};
use actix_web::{Error, HttpRequest, HttpResponse};
use futures::future::Future;
use futures::stream::Stream;
use log::{debug, info};
use uuid::Uuid;

use crate::models::articles::{
    AdminViewRawArticle, DeleteArticlesWithTags, ModifyPublish, QuerySlice, ViewArticle,
};
use crate::{AppState, ArticleList, ArticlesWithTag, EditArticle, NewArticle};
use actix_web::web;

pub struct AdminArticle;

impl AdminArticle {
    pub fn create_article(
        state: Data<AppState>,
        // params: Form<NewArticle>,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("create article");
        extract_form_data!(NewArticle, body, &state)

        // let conn = &state.db.connection();
        // let r = params.into_inner().insert(conn);
        // if r {
        //     api_resp_ok!()
        // } else {
        //     api_resp_err!("create article failed!")
        // }
    }

    pub fn delete_article(
        state: Data<AppState>,
        params: Form<DeleteArticlesWithTags>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("delete article");
        let res = ArticlesWithTag::delete_with_id(
            state.get_ref(),
            Uuid::parse_str(params.id.as_str()).unwrap(),
        );
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
        _req: HttpRequest,
        // params: Form<EditArticle>,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("edit_article");
        extract_form_data!(EditArticle, body, &state)
    }

    pub fn update_publish(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<ModifyPublish>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("update_publish");
        let res = ArticlesWithTag::publish_article(state.get_ref(), &params);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("update_publish failed!"),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("article/new").route(web::post().to_async(AdminArticle::create_article)),
        )
        .service(
            web::resource("article/delete")
                .route(web::delete().to_async(AdminArticle::delete_article)),
        )
        .service(
            web::resource("article/admin/view")
                .route(web::get().to_async(AdminArticle::admin_view_article)),
        )
        // user
        .service(
            web::resource("article/admin/view_raw")
                .route(web::post().to_async(AdminArticle::admin_view_raw_article)),
        )
        .service(
            web::resource("article/admin/view_all")
                .route(web::get().to_async(AdminArticle::admin_list_all_article)),
        )
        .service(
            web::resource("article/edit").route(web::post().to_async(AdminArticle::edit_article)),
        )
        .service(
            web::resource("article/publish")
                .route(web::get().to_async(AdminArticle::update_publish)),
        );
    }
}
