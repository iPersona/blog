use crate::api::recaptcha_api::verify_recaptcha;
use crate::api::InnerContext;
use crate::models::articles::{
    ArticleNumWithTag, ArticleSummary, CommentsResponse, ListAllArticleFilterByTag, QuerySlice,
};
use crate::models::token::{SimpleToken, Token};
use crate::models::user::CheckUser;
use crate::{
    AppState, ArticleList, ArticlesWithTag, Comments, LoginUser, RegisteredUser, UserInfo,
};
use actix_session::Session;
use actix_web::web;
use actix_web::web::Query;
use actix_web::{Error, HttpRequest, HttpResponse};
use futures::Future;
use log::{debug, info};
use uuid::Uuid;
use web::{Data, Form, Path};

pub struct Visitor;

impl Visitor {
    pub fn list_all_article(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Query<QuerySlice>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("list_all_article");
        // let res =
        //     ArticleList::query_list_article(state.get_ref(), params.limit, params.offset, false);
        let conn = &state.db.connection();
        let res = ArticleSummary::list_articles(conn, params.limit, params.offset, false);
        match res {
            Ok(data) => api_resp_data!(data),
            Err(_) => api_resp_err!("list_all_article failed!"),
        }
    }

    fn list_all_article_filter_by_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Query<ListAllArticleFilterByTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("list_all_article_filter_by_tag");
        let is_admin = req.extensions().get::<SimpleToken>().unwrap().is_admin;
        let conn = &state.get_ref().db.connection();
        match ArticleSummary::list_articles_with_tag(
            conn,
            params.tag_id,
            params.limit,
            params.offset,
            is_admin,
        ) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn get_article_number_filter_by_tag(
        state: Data<AppState>,
        req: HttpRequest,
        params: Query<ArticleNumWithTag>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let is_admin = req.extensions().get::<SimpleToken>().unwrap().is_admin;
        let count = ArticleSummary::query_article_numbers_with_tag(&state, params.tag_id, is_admin);
        match count {
            Ok(n) => {
                debug!("article count: {:?}", n);
                api_resp_data!(n)
            }
            Err(e) => api_resp_err!(format!("{:?}", e)),
        }
    }

    fn list_comments(
        state: Data<AppState>,
        req: HttpRequest,
        article_id: Path<Uuid>,
        params: Query<QuerySlice>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        info!("list_comments");
        let permission = req.extensions().get::<InnerContext>().unwrap().permission;
        let (user_id, admin) = match permission {
            Some(0) => {
                let info = UserInfo::from_session(&session);
                match info {
                    Ok(v) => match v {
                        Some(v) => (Some(v.id), true),
                        None => (None, true),
                    },
                    Err(_) => (None, true),
                }
            }
            Some(_) => {
                let info = UserInfo::from_session(&session);
                match info {
                    Ok(v) => match v {
                        Some(v) => (Some(v.id), false),
                        None => (None, false),
                    },
                    Err(_) => (None, false),
                }
            }
            _ => (None, false),
        };
        let pg_pool = state.get_ref().db.into_inner().get().unwrap();
        match Comments::query(
            &pg_pool,
            params.limit,
            params.offset,
            article_id.into_inner(),
        ) {
            Ok(data) => api_resp_data!(CommentsResponse {
                comments: data,
                admin: admin,
                user: user_id,
            }),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn view_article(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Path<Uuid>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("view_article: {:?}", &params);
        let conn = state.db.connection();
        match ArticlesWithTag::query_article(&conn, params.into_inner(), false) {
            Ok(data) => api_resp_data!(data),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn login(
        state: Data<AppState>,
        mut _req: HttpRequest,
        params: Form<LoginUser>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let params = &params.into_inner();

        // verify reCAPTCHA
        let is_ok = verify_recaptcha(params.token().as_str());
        if !is_ok {
            return api_resp_err!("robot detected!");
        }

        let is_remember = params.get_remember();
        let max_age: Option<i64> = if is_remember { Some(24 * 90) } else { None };

        let pg_pool = state.db.into_inner().get().unwrap();
        match params.verification(&pg_pool, &session, &max_age) {
            Ok(user_info) => {
                let token = Token::new(&user_info);
                match token.encode() {
                    Ok(v) => api_resp_data!(v),
                    Err(e) => api_resp_err!(format!("{:?}", e)),
                }
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

    fn create_user(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<RegisteredUser>,
        session: Session,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let pg_pool = state.db.into_inner().get().unwrap();
        match params.into_inner().insert(&pg_pool, &session) {
            Ok(_) => api_resp_ok!(),
            Err(err) => api_resp_err!(&*err),
        }
    }

    fn is_user_exist(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Form<CheckUser>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let pg_pool = state.db.into_inner().get().unwrap();
        let exist = params.into_inner().is_user_exist(&pg_pool);
        api_resp_data!(exist)
    }

    fn get_article_number(
        state: Data<AppState>,
        req: HttpRequest,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        let is_admin = req.extensions().get::<SimpleToken>().unwrap().is_admin;
        let count = ArticleList::query_article_numbers(&state, is_admin);
        match count {
            Ok(n) => {
                debug!("article count: {:?}", n);
                api_resp_data!(n)
            }
            Err(e) => api_resp_err!(format!("{:?}", e)),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("articles").route(web::get().to_async(Visitor::list_all_article)),
        )
        .service(
            web::resource("article/view_comment/{article_id}")
                .route(web::get().to_async(Visitor::list_comments)),
        )
        .service(
            web::resource("article/view/{article_id}")
                .route(web::get().to_async(Visitor::view_article)),
        )
        .service(web::resource("user/login").route(web::post().to_async(Visitor::login)))
        .service(web::resource("user/new").route(web::post().to_async(Visitor::create_user)))
        .service(web::resource("user/exist").route(web::post().to_async(Visitor::is_user_exist)))
        .service(
            web::resource("article/count").route(web::get().to_async(Visitor::get_article_number)),
        )
        .service(
            web::resource("article/tag")
                .route(web::get().to_async(Visitor::list_all_article_filter_by_tag)),
        )
        .service(
            web::resource("article/tag/count")
                .route(web::get().to_async(Visitor::get_article_number_filter_by_tag)),
        );
        //            .service(web::resource("/login_with_github").route(web::get().to_async(Visitor::login_with_github)));
    }
}
