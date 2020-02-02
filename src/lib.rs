#![recursion_limit = "128"]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate ammonia;
extern crate chrono;
extern crate comrak;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hyper;
extern crate hyper_native_tls;
extern crate r2d2;
extern crate r2d2_redis;
extern crate rand;
extern crate redis;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate lettre;
extern crate serde_urlencoded;
extern crate tiny_keccak;
extern crate uuid;

pub mod api;
pub mod cron;
pub mod models;
pub mod schema;
pub mod util;
pub mod web;

pub(crate) use models::UserNotify;
pub(crate) use models::{
    ArticleList, ArticlesWithTag, EditArticle,
    /*ModifyPublish,*/ NewArticle, /*PublishedStatistics,*/
};
pub(crate) use models::{ChangePassword, EditUser, RegisteredUser, UserInfo};
pub(crate) use models::{Comments, DeleteComment, NewComments};
pub(crate) use models::{NewTag, TagCount, Tags};
pub(crate) use schema::{article_tag_relation, article_with_tag, articles, comments, tags, users};
pub use util::RedisPool;
//pub(crate) use util::{get_github_account_nickname_address, get_github_primary_email, get_github_token};
pub use actix::Addr;
pub(crate) use util::{get_password, markdown_render, random_string, sha3_256_encode};
pub use web::{Admin, ArticleWeb};

pub struct AppState {
    pub db: crate::util::postgresql_pool::DataBase,
    pub cache: crate::util::redis_pool::Cache,
    pub cache_worker_addr: actix::Addr<crate::cron::cache::CacheActor>,
}
