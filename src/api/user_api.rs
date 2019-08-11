use crate::api::{ApiResult, InnerContext, JsonResponse};
use crate::models::token::Token;
use crate::models::user::DeleteUser;
use crate::models::user::LoginUser;
use crate::{
    AppState, ArticlesWithTag, ChangePassword, DeleteComment, EditUser, NewComments, UserInfo,
    UserNotify,
};
use actix_web::http::Method;
use actix_web::middleware::session::RequestSession;
use actix_web::{App, Error, Form, HttpRequest, Json};
use futures::Future;

pub struct User;

impl User {
    fn view_user(req: &HttpRequest<AppState>) -> JsonResponse {
        let token = Token::from_request(req);
        if token.is_none() {
            return api_resp_ok!();
        }
        let token = token.unwrap().into_inner();
        let redis_pool = &req.state().cache.into_inner();
        let user_info = UserInfo::view_user_with_cookie(redis_pool, token.as_str());
        api_resp_data!(user_info.as_str())
    }

    fn change_pwd((req, params): (HttpRequest<AppState>, Form<ChangePassword>)) -> JsonResponse {
        let token = Token::from_request(&req);
        if token.is_none() {
            return api_resp_ok!();
        }
        let token = token.unwrap().into_inner();
        let redis_pool = &req.state().cache.into_inner();
        let pg_pool = &req.state().db.into_inner().get().unwrap();
        match params
            .into_inner()
            .change_password(pg_pool, redis_pool, &token)
        {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn edit((req, params): (HttpRequest<AppState>, Form<EditUser>)) -> JsonResponse {
        let token = Token::from_request(&req);
        if token.is_none() {
            return api_resp_ok!();
        }
        let token = token.unwrap().into_inner();
        let redis_pool = &req.state().cache.into_inner();
        let pg_pool = &req.state().db.into_inner().get().unwrap();
        match params.into_inner().edit_user(pg_pool, redis_pool, &token) {
            Ok(num_edit) => api_resp_data!(num_edit),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn sign_out(req: &HttpRequest<AppState>) -> JsonResponse {
        let token = Token::from_request(&req);
        if token.is_none() {
            return api_resp_ok!();
        }
        let token = token.unwrap().into_inner();
        let redis_pool = &req.state().cache.into_inner();
        let res = LoginUser::sign_out(redis_pool, &token);
        if res {
            api_resp_ok!()
        } else {
            api_resp_err!("sign_out failed!")
        }
    }

    fn new_comment((req, params): (HttpRequest<AppState>, Form<NewComments>)) -> JsonResponse {
        let mut params = params.into_inner().clone();
        let token = Token::from_request(&req);
        if token.is_none() {
            return api_resp_ok!();
        }
        let token = token.unwrap().into_inner();
        let redis_pool = &req.state().cache.into_inner();
        let pg_pool = &req.state().db.into_inner().get().unwrap();
        let user =
            serde_json::from_str::<UserInfo>(&UserInfo::view_user_with_cookie(redis_pool, &token))
                .unwrap();
        let admin = UserInfo::view_admin(pg_pool, redis_pool);
        let article =
            ArticlesWithTag::query_without_article(&req.state(), params.article_id(), false)
                .unwrap();
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

        let res = params.insert(&pg_pool, redis_pool, &token);
        if res {
            api_resp_ok!()
        } else {
            api_resp_err!("new_comment failed!")
        }
    }
    fn delete_comment((req, params): (HttpRequest<AppState>, Form<DeleteComment>)) -> JsonResponse {
        let token = Token::from_request(&req);
        if token.is_none() {
            return api_resp_ok!();
        }
        let token = token.unwrap().into_inner();
        let permission = req.extensions().get::<InnerContext>().unwrap().permission;
        let pg_pool = &req.state().db.into_inner().get().unwrap();
        let redis_pool = &req.state().cache.into_inner();
        let res = params
            .into_inner()
            .delete(pg_pool, redis_pool, &token, &permission);
        if res {
            api_resp_ok!()
        } else {
            api_resp_err!("delete_comment failed!")
        }
    }

    pub fn configure(app: App<AppState>) -> App<AppState> {
        app.resource("user/change_pwd", |r| {
            r.method(Method::POST).with(User::change_pwd)
        })
        .resource("user/view", |r| r.get().f(User::view_user))
        .resource("user/sign_out", |r| r.get().f(User::sign_out))
        .resource("user/edit", |r| r.method(Method::POST).with(User::edit))
        .resource("user/new", |r| {
            r.method(Method::POST).with(User::new_comment)
        })
        .resource("user/delete", |r| {
            r.method(Method::POST).with(User::delete_comment)
        })
    }
}
