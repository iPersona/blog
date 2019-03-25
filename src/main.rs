extern crate actix;
extern crate actix_web;
extern crate blog;
extern crate dotenv;
extern crate num_cpus;

use actix::{Addr, SyncArbiter, System};
use actix_web::{
    fs, http,
    http::{header, Method},
    middleware::cors::Cors,
    server, App, HttpRequest, HttpResponse, Result,
};
use blog::util::cookies::Cookies;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate log;

use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::session::{CookieSessionBackend, RequestSession, SessionStorage};
use actix_web::middleware::{Finished, Middleware, Response, Started};
use actix_web::{AsyncResponder, Error};
use blog::models::token::Token;
//use blog::util::get_identity_and_web_context;
use blog::util::postgresql_pool::DataBase;
use blog::util::redis_pool::Cache;
use blog::{Admin, AdminArticle, AdminUser, AppState, ArticleWeb, Tag, Visitor};
use futures::future::{ok, Future};
use futures::sink::Sink;
use std::sync::Arc;
use tera::Context;
use blog::util::get_identity_and_web_context;
use time::Duration;

pub struct Preprocess;

impl Middleware<AppState> for Preprocess {
    fn start(&self, mut req: &HttpRequest<AppState>) -> Result<Started> {
        info!("middleware-start");
        if let Some(token) = Token::get_token(&req.session()) {
            info!("SESSION value: {:?}", token);
            req.extensions_mut().insert(token);
        } /*else {
            let t = Token::new();
            req.session().set("token", t.clone());
            t
        };*/
        info!("path: {:?}", req.path());
        info!("method: {:?}", req.method());

        let ctx = get_identity_and_web_context(req);
        req.extensions_mut().insert(ctx);


        //        else {
        //            info!("NO-SESSION");
        //            let res = req.session().set("counter", 1);
        //            match res {
        //                Ok(_) => info!("success"),
        //                Err(e) => info!("set-session failed: {:?}", e),
        //            };
        //        }
        //        if let Some(token) = req.headers().get("x-token") {
        //            info!("token: {:?}", token);
        //            req.extensions_mut().insert();
        //        }

        Ok(Started::Done)
    }

    fn response(&self, _req: &HttpRequest<AppState>, resp: HttpResponse) -> Result<Response> {
        info!("middleware-response");
        Ok(Response::Done(resp))
    }

    fn finish(&self, req: &HttpRequest<AppState>, resp: &HttpResponse) -> Finished {
        info!("middleware-finish");

        if let Ok(Some(result)) = req.session().get::<String>("token") {
            info!("session value new: {:?}", result);
        } else {
            info!("get session value new failed");
        }

        Finished::Done
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    // 获取环境变量
    dotenv().ok();
    // init logger
    env_logger::init();

    // let mut static_file_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // VSCode调试必须用绝对路径，这里获取不到该CARGO变量值
    let mut static_file_dir = if cfg!(target_os = "macos") {
        "/Users/iPersona/Documents/blog".to_owned()
    } else {
        "/home/omi/Documents/dev/blog".to_owned()
    };
    static_file_dir.push_str("/dist");
    info!("static_file_dir: {}", static_file_dir);

    let sys = System::new("example");
//    let cache_addr = SyncArbiter::start(num_cpus::get(), move || Cache::new());
//    let db_addr = SyncArbiter::start(1 /*num_cpus::get()*/, move || DataBase::new());
//    let cache_addr = Cache::new();
//    let db_addr = DataBase::new();

    server::new(move || {
        let mut app = App::with_state(AppState {
//            db: db_addr,
//            cache: cache_addr,
            db: DataBase::new(),
            cache: Cache::new(),
        });
        app = AdminArticle::configure(app);
//        app = Tag::configure(app);
//        app = AdminUser::configure(app);
        app = app
            .middleware(SessionStorage::new(
                CookieSessionBackend::signed(&[0; 32])
                    .name("blog_session")
                    .secure(false)
                    .max_age(Duration::from_std(std::time::Duration::from_secs(24 * 60 * 60)).unwrap())
            ))
            .middleware(Preprocess);
        app = app.handler(
            "/static",
            fs::StaticFiles::new(static_file_dir.as_str()).unwrap(),
        );
        app
    })
        .bind("0.0.0.0:8888")
        .unwrap()
        .start();
    let _ = sys.run();
}
