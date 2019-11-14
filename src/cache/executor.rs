use crate::models::articles::UpdateArticleVisitNum;
use crate::util::postgresql_pool::DataBase;
use crate::util::redis_pool::Cache;
use actix::{Actor, Handler, Message, SyncContext};
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use uuid::Uuid;

#[derive(Message)]
pub struct IncreaseArticleVisitNum {
    pub article_id: Uuid,
}

pub struct VisitStatisticActor {
    pub day_point: DateTime<Utc>,
    pub db: DataBase,
    pub cache: Cache,
    pub start_time: DateTime<Utc>,
}

impl Default for VisitStatisticActor {
    fn default() -> Self {
        Self {
            day_point: Utc::now(),
            db: DataBase::new(),
            cache: Cache::new(None),
            start_time: Utc::now(),
        }
    }
}

impl VisitStatisticActor {
    pub fn visit_num_key() -> String {
        "visit-cache".to_string()
    }

    pub fn update_day_point(&mut self, new_day_point: DateTime<Utc>) {
        self.day_point = new_day_point;
    }

    pub fn save_visit_num_to_db(&self) {
        let conn = self.db.connection();
        let cache = self.cache.into_inner();
        debug!("save_visit_num_to_db");
        let pool = self.cache.into_inner();
        let info = pool.hgetall::<String>(Self::visit_num_key().as_str());
        let items = UpdateArticleVisitNum::from_strings(info);
        let res = UpdateArticleVisitNum::update_all(items, &conn);
        match res {
            Ok(_) => {}
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }
}

impl Actor for VisitStatisticActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut SyncContext<Self>) {
        info!("visit statistic task actor started up")
    }
}

impl Handler<IncreaseArticleVisitNum> for VisitStatisticActor {
    type Result = ();

    fn handle(&mut self, msg: IncreaseArticleVisitNum, _: &mut SyncContext<Self>) {
        let redis_pool = self.cache.into_inner();

        let redis_key = Self::visit_num_key();
        let hash_key = msg.article_id.to_hyphenated().to_string();
        // increase visit numbers
        redis_pool.hincrby(redis_key.as_str(), hash_key.as_str(), 1);

        // save to database daily
        let now: DateTime<Utc> = Utc::now();
        debug!("now: {:?}, day_point: {:?}", now, self.day_point);
        if now.signed_duration_since(self.day_point).num_days() >= 1 {
            self.update_day_point(now); // update daily statistic
            self.save_visit_num_to_db(); // update visit num to database
        }
    }
}
