use std::env;
use std::sync::Arc;

use diesel::pg::PgConnection;
use dotenv;
use r2d2::PooledConnection;

pub type PgConnectionManager = diesel::r2d2::ConnectionManager<PgConnection>;
pub type PgConnectionPool = diesel::r2d2::Pool<PgConnectionManager>;

pub struct DataBase(pub Arc<PgConnectionPool>);

// impl Actor for DataBase {
//     type Context = SyncContext<Self>;
// }

impl DataBase {
    pub fn new() -> DataBase {
        DataBase(Arc::new(create_pg_pool()))
    }

    pub fn into_inner(&self) -> Arc<PgConnectionPool> {
        self.0.clone()
    }

    pub fn connection(&self) -> PooledConnection<PgConnectionManager> {
        self.0.clone().get().unwrap()
    }
}

fn create_pg_pool() -> PgConnectionPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
