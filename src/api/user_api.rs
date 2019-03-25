use crate::AppState;
use actix_web::{Error, HttpRequest, Json};
use futures::Future;

pub struct User;

impl User {
    fn index(req: &HttpRequest<AppState>) -> Box<Future<Item = Json<ApiResult>, Error = Error>> {
        api_resp_ok()
    }
}
