use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};

pub struct Debug;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Debug
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Transform = DebugMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DebugMiddleware { service })
    }
}

pub struct DebugMiddleware<S> {
    service: S,
}

impl<S, B> Service for DebugMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // TODO: 需要重写
        //        let session = req.get_session();

        //        info!("middleware-start");
        //        if let Some(token) = Token::get_token_from_req(&req) {
        //            info!("SESSION value: {:?}", token);
        //            req.extensions_mut().insert(token);
        //        } /*else {
        //              let t = Token::new();
        //              req.session().set("token", t.clone());
        //              t
        //          };*/
        info!("path: {:?}", req.path());
        info!("method: {:?}", req.method());

        let token = req.headers().get("Authorization");
        match token {
            Some(_v) => {
                // TODO: 解码token
            }
            None => {
                // TODO: 跳转到登录界面
            }
        }
        //
        //        let ctx = get_identity_and_web_context(&mut req);
        //        req.extensions_mut().insert(ctx);
        //
        //        // info!("middleware-finish");
        //        // if let Ok(Some(result)) = req.get_session().get::<String>("token") {
        //        //     info!("session value new: {:?}", result);
        //        // } else {
        //        //     info!("get session value new failed");
        //        // }
        Box::new(self.service.call(req).map(move |res| res))
    }
}
