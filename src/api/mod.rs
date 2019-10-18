#![macro_use]

use crate::util::errors::ErrorCode;

#[derive(Debug, Serialize)]
pub enum Status {
    Ok,
    Err,
}

#[derive(Debug, Serialize)]
#[serde(untagged)] // remove 'ApiResult' level tag, e.g. 'Success', see https://serde.rs/enum-representations.html
pub enum ApiResult {
    Success {
        status: Status,
    },
    Error {
        status: Status,
        code: ErrorCode,
        detail: String,
    },
    Data {
        data: Value,
    },
}

// use actix_web::Error;
// use futures::Future;

// pub type JsonResponse = Box<Future<Item = actix_web::web::Json<ApiResult>, Error = Error>>;
pub type JsonApiResult = actix_web::web::Json<ApiResult>;

/// result macro
// //#[macro_export]
// macro_rules! result_ok {
//     () => {
//         Ok(actix_web::Json($crate::api::ApiResult::Success {
//             status: $crate::api::Status::Ok,
//         }))
//     };
// }

// //#[macro_export]
// macro_rules! result_err {
//     ($detail:expr) => {
//         Ok(actix_web::Json($crate::api::ApiResult::Error {
//             status: $crate::api::Status::Err,
//             detail: String::from($detail),
//         }))
//     };
// }

// //#[macro_export]
// macro_rules! result_data {
//     ($data:expr) => {
//         Ok(actix_web::Json($crate::api::ApiResult::Data {
//             data: serde_json::to_value($data).unwrap(),
//         }))
//     };
// }

/// API response macro
// //#[macro_export]
// macro_rules! api_resp_err {
//     ($detail:expr) => {
//         api_resp!($crate::api::ApiResult::Error {
//             status: $crate::api::Status::Err,
//             detail: String::from($detail)
//         })
//     };
// }

//#[macro_export]
macro_rules! api_resp {
    ($api_ret:expr) => {
        // Box::new(futures::future::ok(actix_web::web::Json($api_ret)))
        futures::future::ok(actix_web::HttpResponse::Ok().json($api_ret))
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
            code: $crate::util::errors::ErrorCode::Unknown,
            detail: String::from($detail)
        })
    };
}

//#[macro_export]
// macro_rules! api_resp_err_with_code {
//     ($code:expr, $detail:expr) => {
//         api_resp!($crate::api::ApiResult::Error {
//             status: $crate::api::Status::Err,
//             code: $code,
//             detail: String::from($detail)
//         })
//     };
// }

//#[macro_export]
macro_rules! api_resp_data {
    ($data:expr) => {
        api_resp!($crate::api::ApiResult::Data {
            data: serde_json::to_value($data).unwrap()
        })
    };
}

macro_rules! extract_form_data {
    ($data_type:ty, $body:expr, $state: expr) => {
        $body
            .map_err(actix_web::Error::from)
            .fold(web::BytesMut::new(), move |mut body, chunk| {
                body.extend_from_slice(&chunk);
                Ok::<_, actix_http::Error>(body)
            })
            .and_then(move |body| {
                use $crate::models::FormDataExtractor;
                let config = serde_qs::Config::new(10, false);
                let data: $data_type = config.deserialize_bytes(&body).unwrap();
                let res = data.execute(&$state);
                match res {
                    Ok(data) => {
                        use typename::TypeName;
                        if data.type_name_of() == "()" {
                            api_resp_ok!()
                        } else {
                            api_resp_data!(data)
                        }
                    }
                    Err(e) => api_resp_err!(e.as_str()),
                }
            })
    };
}

pub mod recaptcha_api;
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
