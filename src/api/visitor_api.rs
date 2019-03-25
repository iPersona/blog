use crate::{AppState, LoginUser};
use actix_web::{Error, HttpRequest, HttpResponse, Json};
use std::future::Future;

pub struct Visitor;

impl Visitor {
    fn login(req: &HttpRequest<AppState>) -> Box<Future<Item = Json<ApiResult>, Error = Error>> {
        api_resp_ok!()
    }
}
