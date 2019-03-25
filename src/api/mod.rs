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

#[derive(Debug, Serialize)]
#[serde(untagged)] // remove 'ApiResult' level tag, e.g. 'Success', see https://serde.rs/enum-representations.html
pub enum APIResult {
    Success { status: Status },
    Error { status: Status, detail: String },
    Data { data: Value },
}

#[derive(Debug, Serialize)]
pub enum Status {
    Ok,
    Err,
}

/// result macro
macro_rules! result_ok {
    () => {
        Ok(Json(ApiResult::Success { status: Status::Ok }))
    };
}

macro_rules! result_err {
    ($detail:expr) => {
        Ok(Json(ApiResult::Error {
            status: Status::Err,
            detail: String::from($detail)
        }))
    };
}

macro_rules! result_data {
    ($data:expr) => {
        Ok(Json(ApiResult::Data {
            data: serde_json::to_value($data).unwrap()
        }))
    };
}


/// API response macro
macro_rules! api_resp_err {
    ($detail:expr) => {
        api_resp!(ApiResult::Error {
            status: Status::Err,
            detail: String::from($detail)
        })
    };
}


macro_rules! api_resp {
    ($api_ret:expr) => {
        Box::new(ok(Json($api_ret)))
    };
}

macro_rules! api_resp_ok {
    () => {
        api_resp!(ApiResult::Success { status: Status::Ok })
    };
}

macro_rules! api_resp_err {
    ($detail:expr) => {
        api_resp!(ApiResult::Error {
            status: Status::Err,
            detail: String::from($detail)
        })
    };
}

macro_rules! api_resp_data {
    ($data:expr) => {
        api_resp!(ApiResult::Data {
            data: serde_json::to_value($data).unwrap()
        })
    };
}
