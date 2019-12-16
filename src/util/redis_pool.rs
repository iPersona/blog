use std::sync::Arc;

use crate::util::env::Env;
use dotenv;
use r2d2;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis;
use std::fs::File;
use std::io::Read;

pub struct RedisPool {
    pool: Pool<RedisConnectionManager>,
    script: Option<redis::Script>,
}

impl RedisPool {
    pub fn new<T>(address: T) -> Self
    where
        T: redis::IntoConnectionInfo,
    {
        let manager = RedisConnectionManager::new(address).unwrap();
        let pool = r2d2::Pool::new(manager).unwrap();
        RedisPool { pool, script: None }
    }

    pub fn new_with_script<T>(address: T, path: &str) -> Self
    where
        T: redis::IntoConnectionInfo,
    {
        let manager = RedisConnectionManager::new(address).unwrap();
        let pool = r2d2::Pool::new(manager).unwrap();
        let mut file = File::open(path).unwrap();
        let mut lua = String::new();
        file.read_to_string(&mut lua).unwrap();
        RedisPool {
            pool,
            script: Some(redis::Script::new(&lua)),
        }
    }

    pub fn keys(&self, pattern: &str) -> Vec<String> {
        redis::cmd("keys")
            .arg(pattern)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn exists(&self, redis_key: &str) -> bool {
        redis::cmd("exists")
            .arg(redis_key)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn expire(&self, redis_key: &str, sec: i64) {
        let a =
            |conn: &redis::Connection| redis::cmd("expire").arg(redis_key).arg(sec).execute(conn);
        self.with_conn(a);
    }

    pub fn del<T>(&self, redis_keys: T) -> bool
    where
        T: redis::ToRedisArgs,
    {
        redis::cmd("del")
            .arg(redis_keys)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn set(&self, redis_key: &str, value: &str) {
        let a =
            |conn: &redis::Connection| redis::cmd("set").arg(redis_key).arg(value).execute(conn);
        self.with_conn(a);
    }

    pub fn get(&self, redis_key: &str) -> Option<String> {
        let res = redis::cmd("get")
            .arg(redis_key)
            .query(&*self.pool.get().unwrap());
        match res {
            Ok(v) => v,
            Err(_) => {
                // not found
                None
            }
        }
    }

    pub fn hset<T>(&self, redis_key: &str, hash_key: &str, value: T)
    where
        T: redis::ToRedisArgs,
    {
        let a = |conn: &redis::Connection| {
            redis::cmd("hset")
                .arg(redis_key)
                .arg(hash_key)
                .arg(value)
                .execute(conn)
        };
        self.with_conn(a);
    }

    pub fn hdel<T>(&self, redis_key: &str, hash_key: T)
    where
        T: redis::ToRedisArgs,
    {
        let a = |conn: &redis::Connection| {
            redis::cmd("hdel")
                .arg(redis_key)
                .arg(hash_key)
                .execute(conn)
        };
        self.with_conn(a)
    }

    pub fn hget<T>(&self, redis_key: &str, hash_key: &str) -> T
    where
        T: redis::FromRedisValue,
    {
        redis::cmd("hget")
            .arg(redis_key)
            .arg(hash_key)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn hgetall<T>(&self, redis_key: &str) -> Vec<T>
    where
        T: redis::FromRedisValue,
    {
        redis::cmd("hgetall")
            .arg(redis_key)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn hincrby(&self, redis_key: &str, hash_key: &str, num: i64) {
        let a = |conn: &redis::Connection| {
            redis::cmd("hincrby")
                .arg(redis_key)
                .arg(hash_key)
                .arg(num)
                .execute(conn)
        };
        self.with_conn(a);
    }

    pub fn hexists(&self, redis_key: &str, hash_key: &str) -> bool {
        redis::cmd("hexists")
            .arg(redis_key)
            .arg(hash_key)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn hlen(&self, redis_key: &str) -> i64 {
        redis::cmd("hlen")
            .arg(redis_key)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn lpush<T>(&self, redis_key: &str, value: T)
    where
        T: redis::ToRedisArgs,
    {
        let a =
            |conn: &redis::Connection| redis::cmd("lpush").arg(redis_key).arg(value).execute(conn);
        self.with_conn(a)
    }

    pub fn llen<T>(&self, redis_key: &str) -> T
    where
        T: redis::FromRedisValue,
    {
        redis::cmd("llen")
            .arg(redis_key)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn ltrim(&self, redis_key: &str, start: i64, stop: i64) {
        let a = |conn: &redis::Connection| {
            redis::cmd("ltrim")
                .arg(redis_key)
                .arg(start)
                .arg(stop)
                .execute(conn)
        };
        self.with_conn(a)
    }

    pub fn lrem<T>(&self, redis_key: &str, count: i64, value: T)
    where
        T: redis::ToRedisArgs,
    {
        let a = |conn: &redis::Connection| {
            redis::cmd("lrem")
                .arg(redis_key)
                .arg(count)
                .arg(value)
                .execute(conn)
        };
        self.with_conn(a)
    }

    pub fn lrange<T>(&self, redis_key: &str, start: i64, stop: i64) -> T
    where
        T: redis::FromRedisValue,
    {
        redis::cmd("lrange")
            .arg(redis_key)
            .arg(start)
            .arg(stop)
            .query(&*self.pool.get().unwrap())
            .unwrap()
    }

    fn with_conn<F: FnOnce(&redis::Connection)>(&self, command: F) {
        command(&*self.pool.get().unwrap());
    }

    pub fn lua_push(&self, redis_key: &str, ip: &str) -> bool {
        self.script
            .as_ref()
            .unwrap()
            .arg(redis_key)
            .arg(ip)
            .invoke::<bool>(&*self.pool.get().unwrap())
            .unwrap()
    }

    pub fn get_redis_key(key: RedisKeys) -> String {
        key.to_string()
    }
}

pub enum RedisKeys {
    VisitCache,
    PersistTime,
}

impl RedisKeys {
    pub fn to_string(&self) -> String {
        match self {
            Self::VisitCache => "visit-cache".to_string(),
            Self::PersistTime => "visit-cache-persist-time".to_string(),
        }
    }
}

fn create_redis_pool(path: Option<&str>) -> RedisPool {
    dotenv::dotenv().ok();

    let database_url = Env::get().redis_url;
    match path {
        Some(path) => RedisPool::new_with_script(database_url.as_str(), path),
        None => RedisPool::new(database_url.as_str()),
    }
}

pub struct Cache(pub Arc<RedisPool>);

// impl Actor for Cache {
//     type Context = SyncContext<Self>;
// }

impl Cache {
    pub fn new(work_dir: Option<&str>) -> Cache {
        match work_dir {
            Some(wd) => {
                let mut path = wd.to_owned();
                path.push_str("/lua/visitor_log.lua");
                Cache(Arc::new(create_redis_pool(Some(path.as_str()))))
            }
            None => Cache(Arc::new(create_redis_pool(None))),
        }
    }

    pub fn into_inner(&self) -> Arc<RedisPool> {
        self.0.clone()
    }
}
