use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, Result};

pub struct Cookies;

impl<S> Middleware<S> for Cookies {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        // TODO: 设置session key
        Ok(Started::Done)
    }
}