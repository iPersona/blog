use std::env;
pub struct Env {
    pub database_url: String,
    pub redis_url: String,
}
impl Env {
    pub fn get() -> Env {
        Env {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        }
    }
}
