#![macro_use]

use crate::util::errors::ErrorCode;
use serde::Serialize;
use serde_json::Value;

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

impl ApiResult {
    pub fn from_data<T>(val: T) -> Self
    where
        T: Serialize,
    {
        ApiResult::Data {
            data: serde_json::to_value(val).unwrap(),
        }
    }
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

macro_rules! middleware_resp_err {
    ($req:expr, $code:expr, $msg:expr) => {
        into_fut_service_response!(
            $req,
            $crate::api::ApiResult::Error {
                status: $crate::api::Status::Err,
                code: $code,
                detail: $msg.to_string()
            }
        )
    };
}

macro_rules! into_fut_service_response {
    ($req:expr, $data:expr) => {
        ok($req.into_response(HttpResponse::Ok().json($data).into_body()))
    };
}

//#[macro_export]
macro_rules! api_resp {
    ($api_ret:expr) => {
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

// #[macro_export]
macro_rules! api_resp_err_with_code {
    ($code:expr, $detail:expr) => {
        api_resp!($crate::api::ApiResult::Error {
            status: $crate::api::Status::Err,
            code: $code,
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

macro_rules! extract_form_data {
    ($data_type:ty, $req:expr, $body:expr, $state: expr) => {
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
                let res = data.execute($req, &$state);
                match res {
                    Ok(data) => {
                        use typename::TypeName;
                        if data.type_name_of() == "()" {
                            api_resp_ok!()
                        } else {
                            api_resp_data!(data)
                        }
                    }
                    Err(e) => api_resp_err_with_code!(e.code, e.detail),
                }
            })
    };
}

/// redirect to specified url
// macro_rules! redirect_to_url {
//     ($url: expr) => {
//         futures::future::ok(
//             actix_web::HttpResponse::Found()
//                 .header(http::header::LOCATION, $url)
//                 .finish(),
//         )
//     };
// }
pub mod article_api;
pub mod comment_api;
pub mod dashboard_api;
pub mod recaptcha_api;
pub mod tag_api;
pub mod user_api;
