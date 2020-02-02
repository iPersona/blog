use crate::models::user::Users;
use crate::util::postgresql_pool::{DataBase, DataBaseConfig};
use actix::{Actor, Handler, Message, SyncContext};
use log::{error, info};

#[derive(Message)]
pub struct ClearUnverifiedUser;

pub struct ClearActor {
    pub db: DataBase,
}

impl Default for ClearActor {
    fn default() -> Self {
        Self {
            db: DataBase::new(DataBaseConfig { max_size: 1 }),
        }
    }
}

impl Actor for ClearActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut SyncContext<Self>) {
        info!("Clear actor started up")
    }
}

impl Handler<ClearUnverifiedUser> for ClearActor {
    type Result = ();

    fn handle(&mut self, _msg: ClearUnverifiedUser, _: &mut SyncContext<Self>) {
        let conn = &self.db.connection();
        let res = Users::clear_unverified(conn);
        match res {
            Ok(_) => {}
            Err(e) => error!("failed to clear unverified user: {}", e),
        }
    }
}
