use crate::api::JsonResponse;
use crate::models::user::{DeleteUser, ViewUserList};
use crate::{AppState, ChangePermission, DisabledUser, UserInfo, Users};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::Method;
use actix_web::{App, AsyncResponder, Form, HttpRequest};
use futures::future::Future;

pub struct AdminUser;

impl AdminUser {
    pub fn delete_user((req, params): (HttpRequest<AppState>, Form<DeleteUser>)) -> JsonResponse {
        let res = Users::delete(&req.state(), params.into_inner().id);
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("delete_user failed!"),
        }
    }

    pub fn view_user_list(req: &HttpRequest<AppState>) -> JsonResponse {
        ViewUserList::new(req.query()).map_or(api_resp_err!("parse query string failed!"), |r| {
            let res = UserInfo::view_user_list(&req.state(), r.limit, r.offset);
            match res {
                Ok(v) => api_resp_data!(v),
                Err(e) => api_resp_err!("view_user_list failed!"),
            }
        })
    }

    pub fn change_permission(
        (req, params): (HttpRequest<AppState>, Form<ChangePermission>),
    ) -> JsonResponse {
        let res = Users::change_permission(&req.state(), params.into_inner());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("change_permission failed!"),
        }
    }

    pub fn change_disabled(
        (req, params): (HttpRequest<AppState>, Form<DisabledUser>),
    ) -> JsonResponse {
        let res = Users::disabled_user(&req.state(), params.into_inner());
        match res {
            Ok(v) => api_resp_data!(v),
            Err(_) => api_resp_err!("change_disable failed!"),
        }
    }

    pub fn configure(app: App<AppState>) -> App<AppState> {
        app.scope("/api/v1/user", |scope| {
            scope
                .resource("/user/view_all", |r| r.get().f(AdminUser::view_user_list))
                .resource("/user/delete", |r| {
                    r.method(Method::POST).with(AdminUser::delete_user)
                })
                .resource("/user/permission", |r| {
                    r.method(Method::POST).with(AdminUser::change_permission)
                })
                .resource("/user/disable", |r| {
                    r.method(Method::POST).with(AdminUser::change_disabled)
                })
        })
    }
}
