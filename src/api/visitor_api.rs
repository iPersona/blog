use crate::api::{ApiResult, InnerContext, JsonResponse};
use crate::models::articles::{
    ArticleSlice, CommentsResponse, ListAllArticleFilterByTag, ListComments, QuerySlice,
    ViewArticle,
};
use crate::models::token::Token;
use crate::{
    AppState, ArticleList, ArticlesWithTag, Comments, LoginUser, RegisteredUser, UserInfo,
};
use actix_web::middleware::session::RequestSession;
use actix_web::{http::Method, FromRequest};
use actix_web::{App, Error, Form, HttpRequest, HttpResponse, Json, Path};
use futures::Future;
use log::info;
use uuid::Uuid;

pub struct Visitor;

impl Visitor {
    // 使用request参数的demo：https://github.com/actix/examples/blob/master/form/src/main.rs
    //    pub fn login((req, params): (HttpRequest<AppState>, Form<LoginUser>)) -> JsonResponse {
    //        info!("access login: {:?}", &params);
    //        api_resp_ok!()
    //    }
    pub fn list_all_article(req: &HttpRequest<AppState>) -> JsonResponse {
        info!("list_all_article");
        QuerySlice::new(req.query()).map_or(api_resp_err!("'id' is not specified!"), |params| {
            let res =
                ArticleList::query_list_article(&req.state(), params.limit, params.offset, false);
            match res {
                Ok(data) => api_resp_data!(data),
                Err(_) => api_resp_err!("list_all_article failed!"),
            }
        })
    }

    fn list_all_article_filter_by_tag(req: &HttpRequest<AppState>) -> JsonResponse {
        info!("list_all_article_filter_by_tag");
        let path_params = Path::<ListAllArticleFilterByTag>::extract(req);
        if path_params.is_err() {
            return api_resp_err!("parse path param: tag_id failed!");
        }
        let path_params = path_params.unwrap();
        let conn = &req.state().db.into_inner().get().unwrap();
        match ArticleList::query_with_tag(conn, path_params.tag_id) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn list_comments(req: &HttpRequest<AppState>) -> JsonResponse {
        info!("list_comments");
        let path_params = Path::<ListComments>::extract(req);
        if path_params.is_err() {
            return api_resp_err!("parse path param: article_id failed!");
        }
        let path_params = path_params.unwrap();
        QuerySlice::new(req.query()).map_or(api_resp_err!("'id' is not specified!"), |params| {
            let permission = req.extensions().get::<InnerContext>().unwrap().permission;
            let (user_id, admin) = match permission {
                Some(0) => {
                    let redis_pool = req.state().cache.into_inner();
                    let token = Token::get_token(&req.session());
                    match token {
                        Some(t) => {
                            let info = serde_json::from_str::<UserInfo>(
                                &UserInfo::view_user_with_cookie(&redis_pool, &t.into_inner()),
                            )
                            .unwrap();
                            (Some(info.id), true)
                        }
                        None => (None, true),
                    }
                }
                Some(_) => {
                    let redis_pool = req.state().cache.into_inner();
                    let token = Token::get_token(&req.session());
                    match token {
                        Some(t) => {
                            let info = serde_json::from_str::<UserInfo>(
                                &UserInfo::view_user_with_cookie(&redis_pool, &t.into_inner()),
                            )
                            .unwrap();
                            (Some(info.id), false)
                        }
                        None => (None, false),
                    }
                }
                _ => (None, false),
            };
            let pg_pool = req.state().db.into_inner().get().unwrap();
            match Comments::query(
                &pg_pool,
                params.limit,
                params.offset,
                path_params.article_id,
            ) {
                Ok(data) => api_resp_data!(CommentsResponse {
                    comments: data,
                    admin: admin,
                    user: user_id,
                }),
                Err(err) => api_resp_err!(&*err),
            }
        })
    }

    fn view_article(req: &HttpRequest<AppState>) -> JsonResponse {
        ViewArticle::new(req.query()).map_or(api_resp_err!("'id' is not specified!"), |params| {
            match ArticlesWithTag::query_without_article(&req.state(), params.id, false) {
                Ok(data) => api_resp_data!(data),
                Err(err) => api_resp_err!(&*err),
            }
        })
    }

    fn login((req, params): (HttpRequest<AppState>, Form<LoginUser>)) -> JsonResponse {
        let params = &params.into_inner();
        let is_remember = params.get_remember();
        let max_age: Option<i64> = if is_remember { Some(24 * 90) } else { None };

        let pg_pool = req.state().db.into_inner().get().unwrap();
        let redis_pool = req.state().cache.into_inner();
        match params.verification(&pg_pool, &redis_pool, &max_age) {
            Ok(token) => {
                token.save_token(&req.session());
                api_resp_ok!()
            }
            Err(err) => api_resp_err!(&*err),
        }
    }

    //    fn login_with_github(req: &mut Request) -> SapperResult<Response> {
    //        let params = get_query_params!(req);
    //        let code = t_param_parse!(params, "code", String);
    //
    //        let redis_pool = req.ext().get::<Redis>().unwrap();
    //        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
    //
    //        let token = get_github_token(&code)?;
    //
    //        let mut response = Response::new();
    //        response.headers_mut().set(ContentType::json());
    //
    //        let (account, nickname, github_address) = get_github_account_nickname_address(&token)?;
    //        match LoginUser::login_with_github(
    //            &pg_pool,
    //            redis_pool,
    //            github_address,
    //            nickname,
    //            account,
    //            &token,
    //        ) {
    //            Ok(cookie) => {
    //                let res = json!({
    //                    "status": true,
    //                });
    //
    //                response.set_status(status::Found);
    //                response.write_body(serde_json::to_string(&res).unwrap());
    //                response.headers_mut().set(Location("/home".to_owned()));
    //
    //                let _ = set_cookie(
    //                    &mut response,
    //                    "blog_session".to_string(),
    //                    cookie,
    //                    None,
    //                    Some("/".to_string()),
    //                    None,
    //                    Some(24),
    //                );
    //            }
    //
    //            Err(err) => {
    //                let res = json!({
    //                    "status": false,
    //                    "error": format!("{}", err)
    //                });
    //
    //                response.write_body(serde_json::to_string(&res).unwrap());
    //            }
    //        }
    //
    //        Ok(response)
    //    }

    fn create_user((req, params): (HttpRequest<AppState>, Form<RegisteredUser>)) -> JsonResponse {
        let pg_pool = req.state().db.into_inner().get().unwrap();
        let redis_pool = req.state().cache.into_inner();

        match params.into_inner().insert(&pg_pool, &redis_pool) {
            Ok(token) => {
                token.save_token(&req.session());
                api_resp_ok!()
            }
            Err(err) => api_resp_err!(&*err),
        }
    }

    pub fn configure(app: App<AppState>) -> App<AppState> {
        app
            // article
            .resource("article/view_all", |r| r.get().f(Visitor::list_all_article))
            .resource("article/view_all/{tag_id}", |r| {
                r.get().f(Visitor::list_all_article_filter_by_tag)
            })
            .resource("article/view_comment/{article_id}", |r| {
                r.get().f(Visitor::list_comments)
            })
            .resource("article/view/{article_id}", |r| {
                r.get().f(Visitor::view_article)
            })
            .resource("article/view/{article_id}", |r| {
                r.get().f(Visitor::view_article)
            })
            // user
            .resource("user/login", |r| {
                r.method(Method::POST).with(Visitor::login)
            })
            .resource("user/new", |r| {
                r.method(Method::POST).with(Visitor::create_user)
            })
        // TODO: 使用github登录
        //        .resource("/login_with_github", |r| {
        //            r.method(Method::Get).with(Visitor::login_with_github)
        //        })
    }
}
