pub mod cli;
pub mod debug_middleware;
pub mod env;
pub mod errors;
pub mod github_information;
pub mod path;
pub mod postgresql_pool;
pub mod redis_pool;
pub use self::redis_pool::RedisPool;

use ammonia::clean;
use comrak::{markdown_to_html, ComrakOptions};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt::Write;
use tiny_keccak::Keccak;

/// Get random value
#[inline]
pub fn random_string(limit: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(limit)
        .collect()
}

/// Convert text to `sha3_256` hex
#[inline]
pub fn sha3_256_encode(s: String) -> String {
    let mut sha3 = Keccak::new_sha3_256();
    sha3.update(s.as_ref());
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
    let mut hex = String::with_capacity(64);
    for byte in res.iter() {
        write!(hex, "{:02x}", byte).expect("Can't fail on writing to string");
    }
    hex
}

/// Convert markdown to html
#[inline]
pub fn markdown_render(md: &str) -> String {
    let option = ComrakOptions {
        ext_strikethrough: true,
        ext_table: true,
        ext_tasklist: true,
        ext_superscript: true,
        safe: true,
        ext_tagfilter: true,
        hardbreaks: true,
        smart: true,
        github_pre_lang: true,
        ..ComrakOptions::default()
    };
    clean(&markdown_to_html(md, &option))
}

/// Get the real password, the first six is a random number
#[inline]
pub fn get_password(raw: &str) -> String {
    let (_, password) = raw.split_at(6);
    password.to_string()
}

// Get visitor's permission and user info
// `0` means Admin
// `1` means User

//pub fn get_identity_and_web_context(_req: &mut ServiceRequest) -> InnerContext {
//    // TODO: 需要重写
//    InnerContext {
//        permission: None,
//        context: Context::new(),
//    }
//    //    let mut web = Context::new();
//    //    let state = req.app_data::<AppState>().unwrap();
//    //    let redis_pool = state.get_ref().cache.into_inner();
//    //    let token = Token::from_session(&req.get_session());
//    //    if token.is_none() {
//    //        return InnerContext {
//    //            permission: None,
//    //            context: web,
//    //        };
//    //    }
//    //    let token = token.unwrap().into_inner();
//    //    if redis_pool.exists(token.as_str()) {
//    //        let info =
//    //            serde_json::from_str::<UserInfo>(&redis_pool.hget::<String>(token.as_str(), "info"))
//    //                .unwrap();
//    //        let group = (&info).groups;
//    //        let notifys = UserNotify::get_notifys(info.id, &redis_pool);
//    //        web.insert("user", &info);
//    //        web.insert("notifys", &notifys);
//    //        InnerContext {
//    //            permission: Some(group),
//    //            context: web,
//    //        }
//    //    } else {
//    //        InnerContext {
//    //            permission: None,
//    //            context: web,
//    //        }
//    //    }
//}

// Get visitor ip information and access time, and then push it to redis key `visitor_log`
//#[inline]
//pub fn visitor_log(req: &Request, redis_pool: &Arc<RedisPool>) {
//    let ip = String::from_utf8(
//        req.headers().get_raw("X-Real-IP").unwrap()[0]
//            .as_slice()
//            .to_vec(),
//    ).unwrap();
//    let timestamp = Utc::now();
//    let redis_pool = redis_pool.clone();
//
//    // https://ipstack.com/documentation
//    thread::spawn(move || {
//        let url = format!("http://api.ipstack.com/{}?access_key=****", &ip);
//        let data = Client::new()
//            .get(&url)
//            .send()
//            .map_err(|e| SapperError::Custom(format!("hyper's io error: '{}'", e)))
//            .and_then(|mut response| {
//                let mut body = String::new();
//                response
//                    .read_to_string(&mut body)
//                    .map(|_| body)
//                    .map_err(|e| SapperError::Custom(format!("read body error: '{}'", e)))
//            })
//            .and_then(|ref body| {
//                #[derive(Deserialize)]
//                struct Inner {
//                    country_name: Option<String>,
//                    region_name: Option<String>,
//                    city: Option<String>,
//                }
//                serde_json::from_str::<Inner>(body)
//                    .map_err(|_| SapperError::Custom(String::from("serde error")))
//                    .map(|inner| inner)
//            })
//            .unwrap();
//        redis_pool.lua_push(
//            "visitor_log",
//            &json!({"ip": &ip, "timestamp": &timestamp, "country_name": data.country_name, "region_name": data.region_name, "city": data.city})
//                .to_string(),
//        );
//    });
//}
//
//pub struct Permissions;
//
//impl Key for Permissions {
//    type Value = Option<i16>;
//}
//
//pub struct WebContext;
//
//impl Key for WebContext {
//    type Value = Context;
//}
