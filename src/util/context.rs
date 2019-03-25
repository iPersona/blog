use crate::{UserInfo, UserNotify};

// TODO： 兼容Sapper框架的Context，后续再看如何修改，前期为了方便框架移植
pub struct Context {
    pub user: Option<UserInfo>,
    pub notifys: Option<Vec<UserNotify>>,
}

impl Context {
    pub fn new(user: Option<UserInfo>, notifys: Option<Vec<UserNotify>>) -> Context {
        Context {
            user,
            notifys,
        }
    }

    pub fn set_user(&mut self, user: Option<UserInfo>) -> &Self {
        self.user = user;
        self
    }

    pub fn set_notifys(&mut self, notifys: Option<Vec<UserNotify>>) -> &Self {
        self.notifys = notifys;
        self
    }
}