use crate::api::InnerContext;
use crate::models::user::LoginUser;
use crate::{
    AppState, ArticlesWithTag, ChangePassword, DeleteComment, EditUser, NewComments, UserInfo,
    UserNotify,
};
use actix_session::Session;
use actix_web::web::{Data, Form};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::Future;

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
        let pg_pool = &state.db.into_inner().get().unwrap();
        match params.into_inner().change_password(pg_pool, &session) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn edit(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<EditUser>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let pg_pool = &state.db.into_inner().get().unwrap();
        match params.into_inner().edit_user(pg_pool, &session) {
            Ok(num_edit) => api_resp_data!(num_edit),
            Err(err) => api_resp_err!(&*err),
        }
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
        _req: HttpRequest,
        params: Form<NewComments>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let mut params = params.into_inner().clone();
        let redis_pool = &state.cache.into_inner();
        let pg_pool = &state.db.into_inner().get().unwrap();
        let user_or = UserInfo::view_user_with_cookie(&session);
        let user: UserInfo;
        match user_or {
            Ok(v) => match v {
                Some(v) => user = v,
                None => return api_resp_err!("failed to get userinfo from current session!"),
            },
            Err(e) => {
                return api_resp_err!(format!(
                    "failed to get userinfo from current session: {:?}",
                    e
                ))
            }
        }
        let admin = UserInfo::view_admin(pg_pool, redis_pool);
        let article =
            ArticlesWithTag::query_without_article(&state, params.article_id(), false).unwrap();
        let reply_user_id = params.reply_user_id();
        match reply_user_id {
            // Reply comment
            Some(reply_user_id) => {
                // Notification replyee
                let user_reply_notify = UserNotify {
                    user_id: reply_user_id,
                    send_user_name: user.nickname.clone(),
                    article_id: article.id,
                    article_title: article.title.clone(),
                    notify_type: "reply".into(),
                };
                user_reply_notify.cache(&redis_pool);

                // If the sender is not an admin and also the responder is also not admin, notify admin
                if reply_user_id != admin.id && user.groups != 0 {
                    let comment_notify = UserNotify {
                        user_id: admin.id,
                        send_user_name: user.nickname.clone(),
                        article_id: article.id,
                        article_title: article.title.clone(),
                        notify_type: "comment".into(),
                    };
                    comment_notify.cache(&redis_pool);
                }
            }
            // Normal comment
            None => {
                if user.groups != 0 {
                    let comment_notify = UserNotify {
                        user_id: admin.id,
                        send_user_name: user.nickname.clone(),
                        article_id: article.id,
                        article_title: article.title.clone(),
                        notify_type: "comment".into(),
                    };
                    comment_notify.cache(&redis_pool);
                }
            }
        }

        let res = params.insert(&pg_pool, &session);
        if res {
            api_resp_ok!()
        } else {
            api_resp_err!("new_comment failed!")
        }
    }

    fn delete_comment(
        state: Data<AppState>,
        req: HttpRequest,
        params: Form<DeleteComment>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let permission = req.extensions().get::<InnerContext>().unwrap().permission;
        let pg_pool = &state.db.into_inner().get().unwrap();
        let res = params.into_inner().delete(pg_pool, &session, &permission);
        if res {
            api_resp_ok!()
        } else {
            api_resp_err!("delete_comment failed!")
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("user/change_pwd").route(web::get().to_async(User::change_pwd)))
            .service(web::resource("user/view").route(web::get().to_async(User::view_user)))
            .service(web::resource("user/sign_out").route(web::get().to_async(User::sign_out)))
            .service(web::resource("user/edit").route(web::post().to_async(User::edit)))
            .service(
                web::resource("user/new_comment").route(web::post().to_async(User::new_comment)),
            )
            .service(
                web::resource("user/delete_comment")
                    .route(web::post().to_async(User::delete_comment)),
            );
    }
}
