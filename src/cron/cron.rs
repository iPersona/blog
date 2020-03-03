use super::cache::LoadUserCache;
use crate::cron::cache::{CacheActor, PersistCache, PersistUncached};
use crate::cron::clear::{ClearActor, ClearUnverifiedUser};
use crate::util::env::Env;
use actix::{Actor, Addr, AsyncContext, Context};
use chrono::{DateTime, Utc};
use std::time::Duration;

pub struct Cron {
    pub cache: Addr<CacheActor>,
    pub clear: Addr<ClearActor>,
    pub start_time: DateTime<Utc>,
}

impl Actor for Cron {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Starting cron process...");

        // persist uncached visit cron into database
        self.cache.do_send(PersistUncached);
        self.cache.do_send(LoadUserCache);
        self.clear.do_send(ClearUnverifiedUser);

        // scheme a task to persist visit cron into database every day
        ctx.run_interval(
            Duration::new(Env::get().persist_cache_interval * 3600, 0),
            persist_cache,
        );
        // scheme a task to clear unverified users
        ctx.run_interval(
            Duration::new(Env::get().clear_unverified_user_interval * 3600, 0),
            clear_unverified_user,
        );
    }
}

/// Persist cache from redis into database
fn persist_cache(cron: &mut Cron, _: &mut Context<Cron>) {
    cron.cache.do_send(PersistCache);
}

/// Clear unverified user
fn clear_unverified_user(cron: &mut Cron, _: &mut Context<Cron>) {
    cron.clear.do_send(ClearUnverifiedUser);
}
