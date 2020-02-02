use std::sync::Arc;

use crate::util::env::Env;
use diesel::pg::PgConnection;
use r2d2::PooledConnection;

pub type PgConnectionManager = diesel::r2d2::ConnectionManager<PgConnection>;
pub type PgConnectionPool = diesel::r2d2::Pool<PgConnectionManager>;

pub struct DataBaseConfig {
    pub max_size: u32,
}

pub struct DataBase(pub Arc<PgConnectionPool>);

impl Default for DataBase {
    fn default() -> Self {
        Self(Arc::new(Self::create_pool(DataBaseConfig { max_size: 10 })))
    }
}

impl DataBase {
    pub fn new(config: DataBaseConfig) -> DataBase {
        DataBase(Arc::new(Self::create_pool(config)))
    }

    pub fn into_inner(&self) -> Arc<PgConnectionPool> {
        self.0.clone()
    }

    pub fn connection(&self) -> PooledConnection<PgConnectionManager> {
        self.into_inner().get().unwrap()
    }

    pub fn create_pool(config: DataBaseConfig) -> PgConnectionPool {
        dotenv::dotenv().ok();

        let database_url = Env::get().database_url;
        let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);
        //        r2d2::Pool::new(manager).expect("Failed to create pool.")

        r2d2::Pool::builder()
            .max_size(config.max_size)
            .build(manager)
            .expect("Failed to create pool!")
    }
}
