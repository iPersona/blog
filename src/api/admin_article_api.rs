use std::sync::Arc;

use actix::{Handler, Message};
use actix_web::actix::Addr;
use actix_web::error::ErrorInternalServerError;
use actix_web::{App, AsyncResponder, Error, Form, HttpRequest};
use futures::future::Future;
use log::{debug, info};
use uuid::Uuid;

use crate::api::JsonResponse;
use crate::models::articles::{
    AdminViewRawArticle, DeleteArticlesWithTags, ModifyPublish, QuerySlice,
};
use crate::util::postgresql_pool::DataBase;
use crate::util::redis_pool::Cache;
use crate::{AppState, ArticleList, ArticlesWithTag, EditArticle, NewArticle, UserNotify};
use actix_web::http::Method;
use actix_web::middleware::session::{RequestSession, Session};
use diesel::pg::PgConnection;

pub struct AdminArticle;

impl AdminArticle {
    pub fn create_article(
        (req, params): (HttpRequest<AppState>, Form<NewArticle>),
    ) -> JsonResponse {
        info!("create article");
        let conn = &req.state().db.into_inner().get().unwrap();
        let r = params.into_inner().insert(conn);
        if r {
            api_resp_ok!()
        } else {
            api_resp_err!("create article failed!")
        }
    }

    pub fn delete_article(
        (req, params): (HttpRequest<AppState>, Form<DeleteArticlesWithTags>),
    ) -> JsonResponse {
        info!("delete article");
        let res = ArticlesWithTag::delete_with_id(
            &req.state(),
            Uuid::parse_str(params.id.as_str()).unwrap(),
        );
        match res {
            Ok(v) => api_resp_data!(v),
            Err(e) => api_resp_err!(&e[..]),
        }
    }

    pub fn admin_view_article(req: &HttpRequest<AppState>) -> JsonResponse {
        info!("admin_view_article");
        req.query()
            .get("id")
            .map_or(api_resp_err!("'id' is not specified!"), |id| {
                debug!("id={:?}", id);
                let res = ArticlesWithTag::query_without_article(
                    &req.state(),
                    Uuid::parse_str(id).unwrap(),
                    true,
                );
                match res {
                    Ok(data) => api_resp_data!(data),
                    Err(e) => api_resp_err!("query_without_article failed!"),
                }
            })
    }

    pub fn admin_view_raw_article(req: &HttpRequest<AppState>) -> JsonResponse {
        info!("admin_view_raw_article");
        AdminViewRawArticle::new(req.query()).map_or(
            api_resp_err!("'id' is not specified!"),
            |params| {
                info!("params:{:?}", params.id.as_str());
                let res = ArticlesWithTag::query_raw_article(
                    &req.state(),
                    Uuid::parse_str(params.id.as_str()).unwrap(),
                );
                match res {
                    Ok(data) => api_resp_data!(data),
                    Err(e) => api_resp_err!("query_raw_article failed!"),
                }
            },
        )
    }

    pub fn admin_list_all_article(req: &HttpRequest<AppState>) -> JsonResponse {
        info!("admin_list_all_article");
        QuerySlice::new(req.query()).map_or(api_resp_err!("'id' is not specified!"), |params| {
            let res =
                ArticleList::query_list_article(&req.state(), params.limit, params.offset, true);
            match res {
                Ok(data) => api_resp_data!(data),
                Err(_) => api_resp_err!("admin_list_all_article failed!"),
            }
        })
    }

    pub fn edit_article((req, params): (HttpRequest<AppState>, Form<EditArticle>)) -> JsonResponse {
        info!("edit_article");
        let res = params.into_inner().edit_article(&req.state());
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("edit_article failed!"),
        }
    }

    pub fn update_publish(
        (req, params): (HttpRequest<AppState>, Form<ModifyPublish>),
    ) -> JsonResponse {
        info!("update_publish");
        let res = ArticlesWithTag::publish_article(&req.state(), &params);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("update_publish failed!"),
        }
    }

    pub fn configure(app: App<AppState>) -> App<AppState> {
      app.resource("article/new", |r| {
            r.method(Method::POST).with(AdminArticle::create_article)
        })
        .resource("article/delete", |r| {
            r.method(Method::POST).with(AdminArticle::delete_article)
        })
        .resource("article/admin/view", |r| {
            r.get().f(AdminArticle::admin_view_article)
        })
        .resource("article//admin/view_raw", |r| {
            r.get().f(AdminArticle::admin_view_raw_article)
        })
        .resource("article/admin/view_all", |r| {
            r.get().f(AdminArticle::admin_list_all_article)
        })
        .resource("article/edit", |r| {
            r.method(Method::POST).with(AdminArticle::edit_article)
        })
        .resource("article/publish", |r| {
            r.method(Method::POST).with(AdminArticle::update_publish)
        })
    }
}
