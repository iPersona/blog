use log::debug;
use std::env;

pub struct Env {
    pub database_url: String,
    pub redis_url: String,
    pub redis_notify_url: String,
    pub token_secret: String,
    pub recaptcha_secret: String,
}

impl Env {
    pub fn get() -> Env {
        Env {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            redis_notify_url: env::var("REDIS_NOTIFY_URL").expect("REDIS_NOTIFY_URL must be set"),
            token_secret: env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set"),
            recaptcha_secret: env::var("RECAPTCHA_SECRET").expect("RECAPTCHA_SECRET must be set"),
        }
    }

    pub fn print(&self) {
        debug!("DATABASE_URL: {}", self.database_url);
        debug!("REDIS_URL: {}", self.redis_url);
        debug!("REDIS_NOTIFY_URL: {}", self.redis_notify_url);
        debug!("TOKEN_SECRET: {}", self.token_secret);
    }
}
