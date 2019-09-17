use crate::models::tag::{DeleteTag};
use crate::{AppState, NewTag, TagCount, Tags};
use actix_web::web::{Data};
use actix_web::{web, web::Form, Error, HttpRequest, HttpResponse};
use futures::future::Future;

pub struct Tag;

impl Tag {
    pub fn create_tag(
        state: Data<AppState>,
        params: Form<NewTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = params.into_inner().insert(state.get_ref());
        match res {
            true => api_resp_ok!(),
            false => api_resp_err!("create_tag failed!"),
        }
    }

    pub fn delete_tag(
        state: Data<AppState>,
        params: Form<DeleteTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
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
        _req: HttpRequest,
        params: Form<Tags>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = params.into_inner().edit_tag(state.get_ref());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("edit_tag failed!"),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/tag/new").route(web::post().to_async(Tag::create_tag)))
            .service(web::resource("/tag/view/count").route(web::get().to_async(Tag::get_tag_with_count)))
            .service(web::resource("/tag/view").route(web::get().to_async(Tag::get_tag_without_count)))
            .service(web::resource("/tag/delete").route(web::delete().to_async(Tag::delete_tag)))
            .service(web::resource("/tag/edit").route(web::post().to_async(Tag::edit_tag)));
    }
}
