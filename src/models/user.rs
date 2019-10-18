use super::super::users;
use super::super::users::dsl::users as all_users;

use super::UserNotify;
use chrono::{Local, NaiveDateTime};
use diesel;
use diesel::prelude::*;
use serde_json;
use std::sync::Arc;
use uuid::Uuid;

use super::super::{
    /*get_github_primary_email, */ get_password, random_string, sha3_256_encode, RedisPool,
};
use crate::AppState;
use actix_session::Session;
use actix_web::Error;
use log::{debug, error};
use std::cell::Ref;
use std::collections::HashMap;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Users {
    pub id: Uuid,
    pub account: String,
    pub github: Option<String>,
    pub password: String,
    pub salt: String,
    pub groups: i16,
    pub nickname: String,
    pub say: Option<String>,
    pub email: String,
    pub disabled: i16,
    pub create_time: NaiveDateTime,
}

impl Users {
    pub fn delete(state: &AppState, id: Uuid) -> Result<usize, String> {
        let conn = &state.db.into_inner().get().unwrap();
        let redis_pool = &state.cache.into_inner();
        let res = diesel::delete(all_users.find(id)).execute(conn);
        match res {
            Ok(data) => {
                UserNotify::remove_with_user(id, redis_pool);
                Ok(data)
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn change_permission(state: &AppState, data: ChangePermission) -> Result<usize, String> {
        let conn = &state.db.into_inner().get().unwrap();
        let res = diesel::update(all_users.filter(users::id.eq(data.id)))
            .set(users::groups.eq(data.permission))
            .execute(conn);
        match res {
            Ok(num_update) => Ok(num_update),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn into_user_info(self) -> UserInfo {
        UserInfo {
            id: self.id,
            account: self.account,
            nickname: self.nickname,
            groups: self.groups,
            say: self.say,
            email: self.email,
            create_time: self.create_time,
            github: self.github,
        }
    }

    pub fn disabled_user(state: &AppState, data: DisabledUser) -> Result<usize, String> {
        let conn = &state.db.into_inner().get().unwrap();
        let res = diesel::update(all_users.filter(users::id.eq(data.id)))
            .set(users::disabled.eq(data.disabled))
            .execute(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn is_user_exist(conn: &PgConnection, email: &str) -> bool {
        let res = all_users
            .filter(users::email.eq(email.to_string()))
            .load::<Users>(conn)
            .expect("Error loading users");
        res.len() > 0
    }
}

#[derive(Insertable, Debug, Clone, Deserialize, Serialize)]
#[table_name = "users"]
struct NewUser {
    pub account: String,
    pub password: String,
    pub salt: String,
    pub nickname: String,
    pub say: Option<String>,
    pub email: String,
    pub github: Option<String>,
}

impl NewUser {
    fn new(reg: RegisteredUser) -> Self {
        let salt = random_string(6);

        NewUser {
            account: reg.account,
            password: sha3_256_encode(get_password(&reg.password) + &salt),
            salt,
            nickname: reg.nickname,
            say: reg.say,
            email: reg.email,
            github: None,
        }
    }

    // TODO: github注册接口
    // fn new_with_github(email: String, github: String, account: String, nickname: String) -> Self {
    //     NewUser {
    //         account,
    //         password: sha3_256_encode(random_string(8)),
    //         salt: random_string(6),
    //         email,
    //         say: None,
    //         nickname,
    //         github: Some(github),
    //     }
    // }

    fn insert(&self, conn: &PgConnection, session: &Session) -> Result<(), String> {
        match diesel::insert_into(users::table)
            .values(self)
            .get_result::<Users>(conn)
        {
            Ok(info) => Ok(self.set_cookies(session, info.into_user_info())),
            Err(err) => Err(format!("{}", err).to_string()),
        }
    }

    fn set_cookies(&self, session: &Session, info: UserInfo) {
        let result = session.set("login_time", Local::now().timestamp());
        if result.is_err() {
            error!("set session value 'login_time' failed!");
        }
        let result = session.set("info", info);
        if result.is_err() {
            error!("set session value 'info' failed!");
        }

        // TODO: 设置超时时间
        // redis_pool.expire(&cookie, 24 * 3600);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisteredUser {
    pub account: String,
    pub password: String,
    pub nickname: String,
    pub say: Option<String>,
    pub email: String,
}

impl RegisteredUser {
    pub fn insert(self, conn: &PgConnection, session: &Session) -> Result<(), String> {
        NewUser::new(self).insert(conn, session)
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub account: String,
    pub nickname: String,
    pub groups: i16,
    pub say: Option<String>,
    pub email: String,
    pub create_time: NaiveDateTime,
    pub github: Option<String>,
}

impl UserInfo {
    pub fn from_session(session: &Session) -> Result<Option<UserInfo>, Error> {
        session.get::<UserInfo>("info")
    }

    pub fn is_admin(&self) -> bool {
        if self.groups == 0 {
            true
        } else {
            false
        }
    }

    pub fn save_to_session(&self, session: &Session) {
        let result = session.set("info", self.clone());
        if result.is_err() {
            error!("set session value 'info' failed!");
        }
    }

    pub fn view_user(conn: &PgConnection, id: Uuid) -> Result<Self, String> {
        let res = all_users
            .select((
                users::id,
                users::account,
                users::nickname,
                users::groups,
                users::say,
                users::email,
                users::create_time,
                users::github,
            ))
            .filter(users::id.eq(id))
            .get_result::<UserInfo>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn view_user_with_cookie(session: &Session) -> Result<Option<UserInfo>, Error> {
        session.get::<UserInfo>("info")
    }

    pub fn view_user_list(state: &AppState, limit: i64, offset: i64) -> Result<Vec<Self>, String> {
        let conn = &state.db.into_inner().get().unwrap();
        let res = all_users
            .select((
                users::id,
                users::account,
                users::nickname,
                users::groups,
                users::say,
                users::email,
                users::create_time,
                users::github,
            ))
            .limit(limit)
            .offset(offset)
            .order(users::create_time)
            .load::<UserInfo>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    /// Get admin information, cache on redis
    /// key is `admin_info`
    pub fn view_admin(conn: &PgConnection, redis_pool: &Arc<RedisPool>) -> Self {
        if redis_pool.exists("admin_info") {
            serde_json::from_str::<UserInfo>(&redis_pool.get("admin_info")).unwrap()
        } else {
            let info = all_users
                .select((
                    users::id,
                    users::account,
                    users::nickname,
                    users::groups,
                    users::say,
                    users::email,
                    users::create_time,
                    users::github,
                ))
                .filter(users::account.eq("admin"))
                .get_result::<UserInfo>(conn)
                .unwrap();
            redis_pool.set("admin_info", &json!(&info).to_string());
            info
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangePassword {
    pub old_password: String,
    pub new_password: String,
}

impl ChangePassword {
    pub fn change_password(&self, conn: &PgConnection, session: &Session) -> Result<usize, String> {
        let user_or = UserInfo::view_user_with_cookie(&session);
        let user: UserInfo;
        match user_or {
            Ok(v) => match v {
                Some(v) => user = v,
                None => return Err("failed to get userinfo from current session!".to_string()),
            },
            Err(e) => {
                return Err(
                    format!("failed to get userinfo from current session: {:?}", e).to_string(),
                )
            }
        }

        if !self.verification(conn, &user.id) {
            return Err("Verification error".to_string());
        }

        let salt = random_string(6);
        let password = sha3_256_encode(get_password(&self.new_password) + &salt);
        let res = diesel::update(all_users.filter(users::id.eq(user.id)))
            .set((users::password.eq(&password), users::salt.eq(&salt)))
            .execute(conn);
        match res {
            Ok(num_update) => Ok(num_update),
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn verification(&self, conn: &PgConnection, id: &Uuid) -> bool {
        let old_user = all_users.filter(users::id.eq(id)).get_result::<Users>(conn);
        match old_user {
            Ok(old) => {
                old.password == sha3_256_encode(get_password(&self.old_password) + &old.salt)
            }
            Err(_) => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditUser {
    pub nickname: String,
    pub say: Option<String>,
    pub email: String,
}

impl EditUser {
    pub fn edit_user(self, conn: &PgConnection, session: &Session) -> Result<usize, String> {
        let user_or = UserInfo::from_session(&session);
        let user: UserInfo;
        match user_or {
            Ok(v) => match v {
                Some(v) => user = v,
                None => return Err("failed to get userinfo from current session!".to_string()),
            },
            Err(e) => {
                return Err(
                    format!("failed to get userinfo from current session: {:?}", e).to_string(),
                )
            }
        }

        let res = diesel::update(all_users.filter(users::id.eq(user.id)))
            .set((
                users::nickname.eq(self.nickname),
                users::say.eq(self.say),
                users::email.eq(self.email),
            ))
            .get_result::<Users>(conn);
        match res {
            Ok(data) => {
                data.into_user_info().save_to_session(session);
                Ok(1)
            }
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangePermission {
    pub id: Uuid,
    pub permission: i16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginUser {
    account: String,
    password: String,
    remember: bool,
    token: String, // reCAPTCHA token
}

impl LoginUser {
    pub fn verification(
        &self,
        conn: &PgConnection,
        session: &Session,
        _max_age: &Option<i64>,
    ) -> Result<UserInfo, String> {
        let res = all_users
            .filter(users::disabled.eq(0))
            .filter(users::account.eq(self.account.to_owned()))
            .get_result::<Users>(conn);
        match res {
            Ok(data) => {
                let pwd = get_password(&self.password);
                debug!("pwd: {:?}", pwd);
                let curpwd = sha3_256_encode(get_password(&self.password) + &data.salt);
                debug!("curpwd: {:?}", curpwd);
                if data.password == sha3_256_encode(get_password(&self.password) + &data.salt) {
                    // TODO: 优化 session 保存时间
                    //                    let ttl = match *max_age {
                    //                        Some(t) => t * 3600,    // 90 days
                    //                        None => 24 * 60 * 60,       // 1 day
                    //                    };
                    let user_info = data.into_user_info();
                    let r = session.set("login_time", Local::now().timestamp());
                    if r.is_err() {
                        error!("set session value 'login_time' failed!");
                    }
                    let r = session.set("info", json!(user_info).to_string());
                    if r.is_err() {
                        error!("set session value 'login_time' failed!");
                    }
                    //                    redis_pool.expire(&cookie, ttl);
                    Ok(user_info)
                } else {
                    Err(String::from("用户或密码错误"))
                }
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn get_remember(&self) -> bool {
        self.remember
    }

    pub fn sign_out(session: &Session) -> bool {
        session.purge();
        true
    }

    //    pub fn login_with_github(
    //        conn: &PgConnection,
    //        redis_pool: &Arc<RedisPool>,
    //        github: String,
    //        nickname: String,
    //        account: String,
    //        token: &str,
    //    ) -> Result<String, String> {
    //        let ttl = 24 * 60 * 60;
    //        match all_users
    //            .filter(users::disabled.eq(0))
    //            .filter(users::github.eq(&github))
    //            .get_result::<Users>(conn)
    //        {
    //            // github already exists
    //            Ok(data) => {
    //                let cookie = sha3_256_encode(random_string(8));
    //                redis_pool.hset(&cookie, "login_time", Local::now().timestamp());
    //                redis_pool.hset(&cookie, "info", json!(data.into_user_info()).to_string());
    //                redis_pool.expire(&cookie, ttl);
    //                Ok(cookie)
    //            }
    //            Err(_) => {
    //                let email = match get_github_primary_email(token) {
    //                    Ok(data) => data,
    //                    Err(e) => return Err(e),
    //                };
    //
    //                match all_users
    //                    .filter(users::disabled.eq(0))
    //                    .filter(users::email.eq(&email))
    //                    .get_result::<Users>(conn)
    //                {
    //                    // Account already exists but not linked
    //                    Ok(data) => {
    //                        let res = diesel::update(all_users.filter(users::id.eq(data.id)))
    //                            .set(users::github.eq(github))
    //                            .get_result::<Users>(conn);
    //                        match res {
    //                            Ok(info) => {
    //                                let cookie = sha3_256_encode(random_string(8));
    //                                redis_pool.hset(&cookie, "login_time", Local::now().timestamp());
    //                                redis_pool.hset(
    //                                    &cookie,
    //                                    "info",
    //                                    json!(info.into_user_info()).to_string(),
    //                                );
    //                                redis_pool.expire(&cookie, ttl);
    //                                Ok(cookie)
    //                            }
    //                            Err(err) => Err(format!("{}", err)),
    //                        }
    //                    }
    //                    // sign up
    //                    Err(_) => NewUser::new_with_github(email, github, account, nickname)
    //                        .insert(conn, redis_pool),
    //                }
    //            }
    //        }
    //    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisabledUser {
    id: Uuid,
    disabled: i16,
}

// DeleteUser
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteUser {
    pub id: Uuid,
}

// View user list
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewUserList {
    pub limit: i64,
    pub offset: i64,
}

impl ViewUserList {
    pub fn new(query: Ref<HashMap<String, String>>) -> Option<ViewUserList> {
        let limit = query
            .get("limit")
            .map_or(-1, |limit| limit.parse::<i64>().unwrap_or_else(|_| -1));
        let offset = query
            .get("offset")
            .map_or(-1, |offset| offset.parse::<i64>().unwrap_or_else(|_| -1));
        if limit == -1 || offset == -1 {
            return None;
        }
        Some(ViewUserList { limit, offset })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckUser {
    pub email: String,
}

impl CheckUser {
    pub fn is_user_exist(self, conn: &PgConnection) -> bool {
        Users::is_user_exist(conn, self.email.as_str())
    }
}
