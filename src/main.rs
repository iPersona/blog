extern crate actix;
extern crate actix_files;
extern crate actix_web;
extern crate base64;
extern crate blog;
extern crate dotenv;
extern crate jsonwebtoken;
extern crate num_cpus;
extern crate strip_markdown;
extern crate typename;

use actix_files as fs;
//use actix_redis::RedisSession;
use actix_web::{App, HttpServer};
use blog::util::env::Env;
use dotenv::dotenv;

extern crate pretty_env_logger;
// #[macro_use]
extern crate log;

extern crate clap;

use actix::{Actor, Arbiter, SyncArbiter, System};
//use actix_web::cookie::SameSite;
use blog::api;
use blog::cron::cache::CacheActor;
use blog::cron::cron::Cron;
use blog::util::cli::Opts;
use blog::util::postgresql_pool::DataBase;
use blog::util::redis_pool::Cache;
// use blog::{AdminArticle, AdminUser, AppState, ChartData, Tag, UserApi, Visitor};
use blog::cron::clear::ClearActor;
use blog::AppState;
use chrono::Utc;
use log::debug;

fn main() {
    ::std::env::set_var("RUST_LOG", "debug,actix_web=debug");
    // ::std::env::set_var("RUST_LOG", "debug");
    // init env variable
    dotenv().ok();
    // init logger
    // env_logger::init();
    pretty_env_logger::init();
    // show env variable
    Env::get().print(true);

    // let mut static_file_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let opt = Opts::new();
    let work_dir = opt.work_dir.clone();
    let mut static_file_dir = opt.work_dir.clone();
    static_file_dir.push_str("/dist");
    debug!("static_file_dir: {}", static_file_dir);

    // Background workers
    let sys = System::builder().stop_on_panic(true).name("blog").build();

    // statistic actor
    let cache_addr = SyncArbiter::start(1, move || CacheActor::default());
    let cache_cron_addr = cache_addr.clone();
    // clear actor
    let clear_addr = SyncArbiter::start(1, move || ClearActor::default());
    let _corn_addr = Cron::start_in_arbiter(&Arbiter::new(), move |_| Cron {
        cache: cache_cron_addr.clone(),
        clear: clear_addr.clone(),
        start_time: Utc::now(),
    });

    //    System::new("example");
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                db: DataBase::default(),
                cache: Cache::new(Some(work_dir.as_str())),
                cache_worker_addr: cache_addr.clone(),
            })
            // TODO: 调试完成后屏蔽掉
            .wrap(blog::util::debug_middleware::Debug)
            .wrap(blog::models::token::PermissionControl)
            .wrap(actix_web::middleware::Logger::default())
            .configure(api::article_api::ArticleApi::configure)
            .configure(api::comment_api::CommentApi::configure)
            .configure(api::tag_api::TagApi::configure)
            .configure(api::user_api::UserApi::configure)
            .configure(api::dashboard_api::DashboardApi::configure)
            .configure(api::mail_box_api::MailboxApi::configure)
            // .wrap(
            //     CookieSession::signed(&[0; 32])
            //         .name("blog_session")
            //         .secure(false)
            //         .max_age(24 * 60 * 60),
            // )
            //            .wrap(
            //                RedisSession::new(Env::get().redis_url.as_str(), &[0; 32])
            //                    .cookie_name("blog_session")
            //                    .ttl(7200) // 保存2小时的cookie数据
            //                    //                .cookie_secure(true)  // TODO: 调试完成后开启
            //                    .cookie_max_age(Duration::hours(24))
            //                    .cookie_same_site(SameSite::Strict), // 禁止跨站传输cookie
            //            )
            .service(fs::Files::new("/", static_file_dir.as_str()).index_file("index.html"))
    })
    .bind("0.0.0.0:8888")
    .unwrap()
    .run();

    let _ = sys.run();
}
