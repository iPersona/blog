extern crate blog;
extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate num_cpus;

use std::env;
use dotenv::dotenv;
use actix::{SyncArbiter, System, Addr};
use actix_web::{
    server,
    App,
    http,
    fs,
    middleware::cors::Cors,
    http::{header, Method}
};
use blog::util::cookies::Cookies;

#[macro_use]
extern crate log;

use blog::{get_identity_and_web_context, Admin, ArticleWeb, Permissions, Postgresql, Redis, WebContext, AppState};
use std::sync::Arc;
use blog::util::redis_pool::Cache;
use blog::util::postgresql_pool::DataBase;

fn main() {
  // 获取环境变量
    dotenv().ok();
    // init logger
    env_logger::init();

    let mut static_file_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    static_file_dir.push_str("/dist");
    info!("static_file_dir: {}", static_file_dir);

    let sys = System::new("example");
    let cache_addr = SyncArbiter::start(num_cpus::get(), move || Cache::new());
    let db_addr = SyncArbiter::start(num_cpus::get(), move || DataBase::new());

    server::new(move || {
        App::with_state(AppState {db: db_addr.clone(), cache: cache_addr.clone()})
            // CORS
          .configure(|app| {
            Cors::for_app(app)
            .allowed_origin("http://192.168.159.131:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
            .register()
          })
            .middleware(Cookies)    // session management
          .handler("/static", fs::StaticFiles::new(static_file_dir.as_str()).unwrap())
    }).bind("0.0.0.0:8088").unwrap().start();
    let _= sys.run();
}
