use crate::api::JsonResponse;
use crate::models::tag::{DeleteTag, ViewTag};
use crate::{AppState, NewTag, TagCount, Tags};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::Method;
use actix_web::{App, AsyncResponder, Form, HttpRequest};
use futures::future::Future;

pub struct Tag;

impl Tag {
    pub fn create_tag((req, params): (HttpRequest<AppState>, Form<NewTag>)) -> JsonResponse {
        let res = params.into_inner().insert(&req.state());
        match res {
            true => api_resp_ok!(),
            false => api_resp_err!("create_tag failed!"),
        }
    }

    pub fn delete_tag((req, params): (HttpRequest<AppState>, Form<DeleteTag>)) -> JsonResponse {
        let res = Tags::delete_tag(&req.state(), params.into_inner().id);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("delete_tag failed!"),
        }
    }

    pub fn view_tag(req: &HttpRequest<AppState>) -> JsonResponse {
        ViewTag::new(req.query()).map_or(api_resp_err!("parse query string failed!"), |params| {
            let res = TagCount::view_all_tag_count(&req.state(), params.limit, params.offset);
            match res {
                Ok(v) => api_resp_data!(v),
                Err(_) => api_resp_err!("delete_tag failed!"),
            }
        })
    }

    pub fn edit_tag((req, params): (HttpRequest<AppState>, Form<Tags>)) -> JsonResponse {
        let res = params.into_inner().edit_tag(&req.state());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("edit_tag failed!"),
        }
    }

    pub fn configure(app: App<AppState>) -> App<AppState> {
        app.scope("/api/v1/tag", |scope| {
            scope
                .resource("/new", |r| r.method(Method::POST).with(Tag::create_tag))
                .resource("/view", |r| r.get().f(Tag::view_tag))
                .resource("/delete", |r| r.method(Method::POST).with(Tag::delete_tag))
                .resource("/edit", |r| r.method(Method::POST).with(Tag::edit_tag))
        })
    }
}
