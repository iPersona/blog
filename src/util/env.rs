use log::debug;
use std::env;

pub struct Env {
    pub database_url: String,
    pub redis_url: String,
    pub token_secret: String,
    pub recaptcha_secret: String,
    pub email: String,
    pub domain: String,
    pub smtp_server: String,
    pub login_token_expired: i64,
    pub verify_token_expired: i64,

    // background interval (hour)
    // persist cache interval
    pub persist_cache_interval: u64,
    //
    pub clear_unverified_user_interval: u64,
}

impl Env {
    pub fn get() -> Env {
        Env {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            token_secret: env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set"),
            recaptcha_secret: env::var("RECAPTCHA_SECRET").expect("RECAPTCHA_SECRET must be set"),
            email: env::var("EMAIL").expect("EMAIL must be set"),
            domain: env::var("DOMAIN").expect("DOMAIN must be set"),
            smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"),
            login_token_expired: env::var("LOGIN_TOKEN_EXPIRED")
                .expect("LOGIN_TOKEN_EXPIRED must be set")
                .parse::<i64>()
                .unwrap(),
            verify_token_expired: env::var("VERIFY_TOKEN_EXPIRED")
                .expect("VERIFY_TOKEN_EXPIRED must be set")
                .parse::<i64>()
                .unwrap(),
            persist_cache_interval: env::var("PERSIST_CACHE_INTERVAL")
                .expect("PERSIST_CACHE_INTERVAL must be set")
                .parse::<u64>()
                .unwrap(),
            clear_unverified_user_interval: env::var("CLEAR_UNVERIFIED_USER_INTERVAL")
                .expect("CLEAR_UNVERIFIED_USER_INTERVAL must be set")
                .parse::<u64>()
                .unwrap(),
        }
    }

    pub fn print(&self, basic: bool) {
        let mut env_str = String::new();
        env_str.push_str("\n");
        env_str.push_str(format!("DATABASE_URL: {}\n", self.database_url).as_str());
        env_str.push_str(format!("REDIS_URL: {}\n", self.redis_url).as_str());

        // secret info
        if !basic {
            env_str.push_str(format!("TOKEN_SECRET: {}\n", self.token_secret).as_str());
            env_str.push_str(format!("RECAPTCHA_SECRET: {}\n", self.recaptcha_secret).as_str());
        }

        env_str.push_str(format!("EMAIL: {}\n", self.email).as_str());
        env_str.push_str(format!("DOMAIN: {}\n", self.domain).as_str());
        env_str.push_str(format!("SMTP_SERVER: {}\n", self.smtp_server).as_str());
        env_str.push_str(format!("LOGIN_TOKEN_EXPIRED: {}h\n", self.login_token_expired).as_str());
        env_str
            .push_str(format!("VERIFY_TOKEN_EXPIRED: {}h\n", self.verify_token_expired).as_str());
        env_str.push_str(
            format!("PERSIST_CACHE_INTERVAL: {}h\n", self.persist_cache_interval).as_str(),
        );
        env_str.push_str(
            format!(
                "CLEAR_UNVERIFIED_USER_INTERVAL: {}h\n",
                self.clear_unverified_user_interval
            )
            .as_str(),
        );

        debug!("{}", env_str)
    }
}
