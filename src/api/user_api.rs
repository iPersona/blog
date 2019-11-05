use crate::models::user::LoginUser;
use crate::{AppState, ChangePassword, DeleteComment, EditUser, NewComments, UserInfo};
use actix_session::Session;
use actix_web::web::{Data, Form};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::stream::Stream;
use futures::Future;
use log::debug;

pub struct User;

impl User {
    fn view_user(
        _state: Data<AppState>,
        _req: HttpRequest,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let user_info = UserInfo::view_user_with_cookie(&session);
        match user_info {
            Ok(v) => match v {
                Some(v) => api_resp_data!(v),
                None => api_resp_err!("can not get userinfo from session!"),
            },
            Err(e) => api_resp_err!(format!("can not get userinfo from session: {:?}", e)),
        }
    }

    fn change_pwd(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<ChangePassword>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let pg_pool = &state.db.connection();
        match params.into_inner().change_password(pg_pool, &session) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn edit(
        state: Data<AppState>,
        req: HttpRequest,
        body: web::Payload,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("edit_user");
        extract_form_data!(EditUser, req, body, &state)
    }

    fn sign_out(
        _state: Data<AppState>,
        _req: HttpRequest,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let res = LoginUser::sign_out(&session);
        if res {
            api_resp_ok!()
        } else {
            api_resp_err!("sign_out failed!")
        }
    }

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

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("user/change_pwd").route(web::get().to_async(User::change_pwd)))
            .service(web::resource("user/view").route(web::get().to_async(User::view_user)))
            .service(web::resource("user/sign_out").route(web::get().to_async(User::sign_out)))
            .service(web::resource("user/edit").route(web::post().to_async(User::edit)))
            .service(
                web::resource("user/comment/new").route(web::post().to_async(User::new_comment)),
            )
            .service(
                web::resource("user/delete_comment")
                    .route(web::post().to_async(User::delete_comment)),
            );
    }
}
