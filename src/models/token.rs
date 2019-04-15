use uuid::Uuid;
use actix_web::HttpRequest;
use actix_web::middleware::session::{CookieSessionBackend, RequestSession, SessionStorage, Session};
use crate::AppState;
use log::debug;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token(String);

impl Token {
    pub fn new() -> Token {
        Token(Uuid::new_v4().simple().to_string())
    }

    pub fn into_inner(&self) -> String {
        self.0.clone()
    }

    pub fn from_request(req: &HttpRequest<AppState>) -> Option<Token> {
        match req.session().get::<String>("token") {
            Ok(Some(v)) => Some(Token(v.clone())),
            _ => None,
        }
    }

    pub fn save_token(&self, session: &Session) {
        session.set("token", self.clone());
    }

    pub fn get_token(session: &Session) -> Option<Self> {
        match session.get::<Self>("token") {
            Ok(Some(v)) => Some(v),
            Ok(None) => {
                debug!("no token found in session!");
                None
            },
            Err(err) => {
                debug!("get session token failed: {:?}",  err);
                None
            },
        }
    }
}

impl From<String> for Token {
    fn from(v: String) -> Self {
        Token(v)
    }
}
