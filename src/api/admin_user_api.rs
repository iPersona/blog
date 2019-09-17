use crate::models::user::{DeleteUser, ViewUserList};
use crate::{AppState, ChangePermission, DisabledUser, UserInfo, Users};
use actix_web::web::{Data, Form, Query};
use actix_web::HttpRequest;
use actix_web::{web, Error, HttpResponse};
use futures::future::Future;

pub struct AdminUser;

impl AdminUser {
    pub fn delete_user(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<DeleteUser>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = Users::delete(state.get_ref(), params.into_inner().id);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("delete_user failed!"),
        }
    }

    pub fn view_user_list(
        state: Data<AppState>,
        _req: HttpRequest,
        query: Query<ViewUserList>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = UserInfo::view_user_list(state.get_ref(), query.limit, query.offset);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(e) => api_resp_err!(format!("view_user_list failed: {:?}", e)),
        }
    }

    pub fn change_permission(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<ChangePermission>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = Users::change_permission(state.get_ref(), params.into_inner());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("change_permission failed!"),
        }
    }

    pub fn change_disabled(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<DisabledUser>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = Users::disabled_user(state.get_ref(), params.into_inner());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("change_disable failed!"),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/user/view_all").route(web::get().to_async(AdminUser::view_user_list)),
        )
        .service(web::resource("/user/delete").route(web::post().to_async(AdminUser::delete_user)))
        .service(
            web::resource("/user/permission")
                .route(web::post().to_async(AdminUser::change_permission)),
        )
        .service(
            web::resource("/user/disable").route(web::post().to_async(AdminUser::change_disabled)),
        );
    }
}
