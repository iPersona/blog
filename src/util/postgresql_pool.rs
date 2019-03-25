use std::env;
use std::sync::Arc;

use diesel::pg::PgConnection;
use dotenv;
use diesel::r2d2;
use actix::{Actor, SyncContext};

pub type PgConnectionManager = r2d2::ConnectionManager<PgConnection>;
pub type PgConnectionPool = r2d2::Pool<PgConnectionManager>;

pub struct DataBase(pub PgConnectionPool);

impl Actor for DataBase {
    type Context = SyncContext<Self>;
}

impl DataBase {
    pub fn new() -> DataBase {
        DataBase(create_pg_pool())
    }
}

fn create_pg_pool() -> PgConnectionPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}

