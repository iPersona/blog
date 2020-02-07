use crate::models::articles::UpdateArticleVisitNum;
use crate::models::daily_statistic::InsertDailyStatistic;
use crate::util::postgresql_pool::{DataBase, DataBaseConfig};
use crate::util::redis_pool::{Cache, RedisKeys};
use actix::{Actor, Handler, Message, SyncContext};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, error, info};
use time::Duration;
use uuid::Uuid;

#[derive(Message)]
pub struct IncreaseArticleVisitNum {
    pub article_id: Uuid,
}

pub struct CacheActor {
    pub db: DataBase,
    pub cache: Cache,
    pub start_time: DateTime<Utc>,
}

impl Default for CacheActor {
    fn default() -> Self {
        Self {
            db: DataBase::new(DataBaseConfig { max_size: 10 }),
            cache: Cache::new(None),
            start_time: Utc::now(),
        }
    }
}

impl CacheActor {
    pub fn save_visit_num_to_db(&self, time: NaiveDateTime) {
        let conn = self.db.connection();
        let redis = self.cache.into_inner();
        debug!("save_visit_num_to_db");
        let pool = self.cache.into_inner();
        let info = pool.hgetall::<String>(RedisKeys::VisitCache.to_string().as_str());
        let items = UpdateArticleVisitNum::from_strings(info);
        let res = UpdateArticleVisitNum::update_all(items, &conn);
        match res {
            Ok(_) => {}
            Err(e) => error!("{:?}", e),
        };

        // save daily statistic to db
        let res = InsertDailyStatistic::insert(&conn, &redis, time);
        match res {
            Ok(_) => {}
            Err(e) => error!("{:?}", e),
        }
    }

    pub fn clear_visit_cache(&self) {
        let redis = self.cache.into_inner();
        let is_ok = redis.del(RedisKeys::VisitCache.to_string().as_str());
        if !is_ok {
            error!("del {} failed!", RedisKeys::VisitCache.to_string().as_str());
        }
    }

    /// Save the persist time,
    /// used for persist visit cron data into database when the server exist unexpected
    pub fn save_persist_time(&self) {
        let redis = self.cache.into_inner();
        redis.set(
            RedisKeys::PersistTime.to_string().as_str(),
            Utc::now().naive_utc().to_string().as_str(),
        );
    }

    pub fn persist_time(&self) -> Option<NaiveDateTime> {
        let redis = self.cache.into_inner();
        let time = redis.get(RedisKeys::PersistTime.to_string().as_str());
        match time {
            Some(t) => Some(NaiveDateTime::parse_from_str(t.as_str(), "%Y-%m-%d %H:%M:%S%.f").unwrap()),
            None => None,
        }
    }

    fn is_cache_empty(&self) -> bool {
        let redis = self.cache.into_inner();
        let key_nums = redis.hlen(RedisKeys::VisitCache.to_string().as_str());
        key_nums <= 0
    }
}

impl Actor for CacheActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut SyncContext<Self>) {
        info!("Cache actor started up")
    }
}

impl Handler<IncreaseArticleVisitNum> for CacheActor {
    type Result = ();

    fn handle(&mut self, msg: IncreaseArticleVisitNum, _: &mut SyncContext<Self>) {
        let redis_pool = self.cache.into_inner();

        let redis_key = RedisKeys::VisitCache.to_string();
        let hash_key = msg.article_id.to_hyphenated().to_string();
        // increase visit numbers
        redis_pool.hincrby(redis_key.as_str(), hash_key.as_str(), 1);
    }
}

#[derive(Message)]
pub struct PersistCache;

impl Handler<PersistCache> for CacheActor {
    type Result = ();

    fn handle(&mut self, _msg: PersistCache, _: &mut SyncContext<Self>) {
        self.save_visit_num_to_db(Utc::now().naive_utc()); // update visit num to database
        self.save_persist_time(); // update persist time
        self.clear_visit_cache(); // reset daily statistic of redis
    }
}

#[derive(Message)]
pub struct PersistUncached;

impl Handler<PersistUncached> for CacheActor {
    type Result = ();

    fn handle(&mut self, _msg: PersistUncached, _: &mut SyncContext<Self>) {
        if self.is_cache_empty() {
            return;
        }

        let persist_time = self.persist_time();
        match persist_time {
            Some(t) => {
                let yesterday = t + Duration::days(1); // store to next day,

                // Save cache data into database then clear the cache data
                self.save_visit_num_to_db(yesterday);
                self.save_persist_time();
                self.clear_visit_cache();
            }
            None => {
                let t = Utc::now().naive_utc();

                // Save cache data into database then clear the cache data
                self.save_visit_num_to_db(t);
                self.save_persist_time();
                self.clear_visit_cache();
            }
        }
    }
}
