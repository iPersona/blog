use crate::cache::executor::{PersistCache, PersistUncached, VisitStatisticActor};
use actix::{Actor, Addr, AsyncContext, Context};
use chrono::{DateTime, Utc};
use log::info;
use std::time::Duration;

pub struct Cron {
    pub executor: Addr<VisitStatisticActor>,
    pub start_time: DateTime<Utc>,
}

impl Cron {
    pub fn new(executor: Addr<VisitStatisticActor>) -> Self {
        Self {
            executor,
            start_time: Utc::now(),
        }
    }
}

impl Actor for Cron {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Starting cron process...");

        // persist uncached visit cache into database
        self.executor.do_send(PersistUncached);

        // scheme a task to persist visit cache into database every day
        ctx.run_interval(Duration::new(24 * 3600, 0), persist_cache);
    }
}

fn persist_cache(cron: &mut Cron, _: &mut Context<Cron>) {
    cron.executor.do_send(PersistCache);
}
