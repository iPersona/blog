use crate::schema::daily_statistic;
use crate::util::redis_pool::RedisKeys;
use crate::RedisPool;
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use log::error;
use std::sync::Arc;
use typename::TypeName;

#[derive(Insertable, Debug, Clone, Deserialize, Serialize)]
#[table_name = "daily_statistic"]
pub struct InsertDailyStatistic {
    pub visit_num: i64,
    pub today: NaiveDateTime,
}

impl InsertDailyStatistic {
    pub fn insert(
        conn: &PgConnection,
        redis: &Arc<RedisPool>,
        time: NaiveDateTime,
    ) -> Result<bool, String> {
        let res = DailyStatistic::get_today_from_cache(redis)?;
        let res = diesel::insert_into(daily_statistic::table)
            .values(Self {
                visit_num: res.visit_num,
                today: time,
            })
            .execute(conn);
        match res {
            Ok(n) => {
                if n > 0 {
                    Ok(true)
                } else {
                    Err("no row inserted!".to_string())
                }
            }
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

#[derive(Insertable, Debug, Clone, Deserialize, Serialize, TypeName, Queryable, QueryableByName)]
#[table_name = "daily_statistic"]
pub struct DailyStatistic {
    pub today: NaiveDateTime,
    pub visit_num: i64,
}

impl DailyStatistic {
    pub fn get_period(
        conn: &PgConnection,
        redis: &Arc<RedisPool>,
        start: &str,
        end: &str,
    ) -> Result<Vec<Self>, String> {
        let mut data = Vec::new();

        let raw_sql = format!(
            "SELECT * FROM daily_statistic WHERE today BETWEEN '{}' AND '{}' ORDER BY today ASC",
            start, end
        );
        let period = diesel::sql_query(raw_sql)
            .load::<Self>(conn)
            .map_err(|e| format!("get_period failed: {}", e))?;
        data.extend_from_slice(period.as_slice());

        if !Self::is_today_recorded(conn) {
            // append today data if not saved into db yet
            let today = Self::get_today_from_cache(redis)?;
            data.push(today);
        }

        Ok(data)
    }

    pub fn get_today(conn: &PgConnection, redis: &Arc<RedisPool>) -> Result<Self, String> {
        if Self::is_today_recorded(conn) {
            // saved to db, just read from db
            Self::get_today_from_db(conn)
        } else {
            // still in cache state, read from cache
            Self::get_today_from_cache(redis)
        }
    }

    pub fn is_today_recorded(conn: &PgConnection) -> bool {
        // use diesel::sql_types::BigInt;

        // #[derive(QueryableByName)]
        // struct Count {
        //     #[sql_type = "BigInt"]
        //     count: i64,
        // }

        // let raw_sql = "select count(*) from daily_statistic where date(now()) = date(today)";
        // let res = diesel::sql_query(raw_sql).load::<Count>(conn);
        // match res {
        //     Ok(c) => {
        //         if c[0].count > 0 {
        //             true
        //         } else {
        //             false
        //         }
        //     }
        //     Err(e) => {
        //         error!("is_today_recorded failed: {:?}", e);
        //         false
        //     }
        // }

        use crate::schema::daily_statistic::dsl::*;

        let latest = daily_statistic
            .select(today)
            .order(today.desc())
            .first::<NaiveDateTime>(conn);
        match latest {
            Ok(t) => {
                let now = Utc::now().naive_utc();
                return if now.signed_duration_since(t).num_days() > 1 {
                    true
                } else {
                    false
                };
            }
            Err(e) => {
                error!("get latest insert time failed: {}", e);
                return false;
            }
        }
    }

    pub fn get_today_from_cache(redis: &Arc<RedisPool>) -> Result<Self, String> {
        let mut str_list = redis.hgetall::<String>(RedisKeys::VisitCache.to_string().as_str());
        let mut sum: i64 = 0;
        while str_list.len() > 0 {
            let num_str = str_list.pop().unwrap();
            sum += num_str.as_str().parse::<i64>().unwrap();
            let _ = str_list.pop();
        }

        Ok(Self {
            today: Utc::now().naive_utc(),
            visit_num: sum,
        })
    }

    pub fn get_today_from_db(conn: &PgConnection) -> Result<Self, String> {
        use crate::schema::daily_statistic::dsl::*;

        let res = daily_statistic
            .select((today, visit_num))
            .order(today.desc())
            .first::<(NaiveDateTime, i64)>(conn);
        match res {
            Ok((t, v)) => Ok(Self {
                today: t,
                visit_num: v,
            }),
            Err(e) => Err(format!("get_today_from_db failed: {}", e)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Period {
    pub start: String,
    pub end: String,
}
