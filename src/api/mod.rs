#![macro_use]

#[derive(Debug, Serialize)]
pub enum Status {
    Ok,
    Err,
}

#[derive(Debug, Serialize)]
#[serde(untagged)] // remove 'ApiResult' level tag, e.g. 'Success', see https://serde.rs/enum-representations.html
pub enum ApiResult {
    Success { status: Status },
    Error { status: Status, detail: String },
    Data { data: Value },
}

use actix_web::{Error, Json};
use futures::Future;

pub type JsonResponse = Box<Future<Item = Json<ApiResult>, Error = Error>>;

/// result macro
//#[macro_export]
macro_rules! result_ok {
    () => {
        Ok(actix_web::Json($crate::api::ApiResult::Success {
            status: $crate::api::Status::Ok,
        }))
    };
}

//#[macro_export]
macro_rules! result_err {
    ($detail:expr) => {
        Ok(actix_web::Json($crate::api::ApiResult::Error {
            status: $crate::api::Status::Err,
            detail: String::from($detail),
        }))
    };
}

//#[macro_export]
macro_rules! result_data {
    ($data:expr) => {
        Ok(actix_web::Json($crate::api::ApiResult::Data {
            data: serde_json::to_value($data).unwrap(),
        }))
    };
}

/// API response macro
//#[macro_export]
macro_rules! api_resp_err {
    ($detail:expr) => {
        api_resp!($crate::api::ApiResult::Error {
            status: $crate::api::Status::Err,
            detail: String::from($detail)
        })
    };
}

//#[macro_export]
macro_rules! api_resp {
    ($api_ret:expr) => {
        Box::new(futures::future::ok(actix_web::Json($api_ret)))
    };
}

//#[macro_export]
macro_rules! api_resp_ok {
    () => {
        api_resp!($crate::api::ApiResult::Success {
            status: $crate::api::Status::Ok
        })
    };
}

//#[macro_export]
macro_rules! api_resp_err {
    ($detail:expr) => {
        api_resp!($crate::api::ApiResult::Error {
            status: $crate::api::Status::Err,
            detail: String::from($detail)
        })
    };
}

//#[macro_export]
macro_rules! api_resp_data {
    ($data:expr) => {
        api_resp!($crate::api::ApiResult::Data {
            data: serde_json::to_value($data).unwrap()
        })
    };
}

pub mod user_api;
pub mod visitor_api;

pub use self::user_api::User;
pub use self::visitor_api::Visitor;

pub mod admin_article_api;
pub mod admin_tag_api;
pub mod admin_user_api;

pub use self::admin_article_api::AdminArticle;
pub use self::admin_tag_api::Tag;
pub use self::admin_user_api::AdminUser;

pub mod admin_chart_data_api;

pub use self::admin_chart_data_api::ChartData;
use serde_json::Value;
use tera::Context;

#[derive(Debug, Clone, Serialize)]
pub struct InnerContext {
    pub permission: Option<i16>,
    pub context: Context,
}
