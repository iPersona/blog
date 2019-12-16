use super::super::users;
use super::super::users::dsl::users as all_users;

use super::FormDataExtractor;
use super::UserNotify;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use serde_json;
use std::sync::Arc;
use uuid::Uuid;

use super::super::{
    /*get_github_primary_email, */ get_password, random_string, sha3_256_encode, RedisPool,
};
use super::token::Token;
use crate::models::token::TokenExtension;
use crate::AppState;
use log::debug;
use std::cell::Ref;
use std::collections::HashMap;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Users {
    pub id: Uuid,
    pub account: String,
    pub password: String,
    pub salt: String,
    pub groups: i16,
    pub nickname: String,
    pub say: Option<String>,
    pub email: String,
    pub disabled: i16,
    pub create_time: NaiveDateTime,
    pub github: Option<String>,
}

impl Users {
    pub fn delete(state: &AppState, id: Uuid) -> Result<usize, String> {
        let conn = &state.db.connection();
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
        let conn = &state.db.connection();
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
        let conn = &state.db.connection();
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

    fn insert(&self, state: &AppState) -> Result<(), String> {
        let conn = state.db.connection();
        match diesel::insert_into(users::table)
            .values(self)
            .get_result::<Users>(&conn)
        {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{}", err).to_string()),
        }
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
    pub fn insert(self, state: &AppState) -> Result<(), String> {
        NewUser::new(self).insert(state)
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
    pub fn from_token(token: &Token) -> Self {
        UserInfo {
            id: Uuid::parse_str(token.user_id.as_str()).unwrap(),
            account: token.user_name.clone(),
            nickname: token.user_nickname.clone(),
            groups: if token.is_admin { 0 } else { 1 },
            say: None,
            email: token.email.clone(),
            create_time: token.user_create_time,
            github: None,
        }
    }

    pub fn is_admin(&self) -> bool {
        if self.groups == 0 {
            true
        } else {
            false
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

    pub fn view_user_list(state: &AppState, limit: i64, offset: i64) -> Result<Vec<Self>, String> {
        let conn = &state.db.connection();
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
            serde_json::from_str::<UserInfo>(&redis_pool.get("admin_info").unwrap()).unwrap()
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
    pub fn change_password(
        &self,
        user_info: &UserInfo,
        conn: &PgConnection,
    ) -> Result<usize, String> {
        if !self.verification(conn, &user_info.id) {
            return Err("Verification error: old password is not correct".to_string());
        }

        let salt = random_string(6);
        let password = sha3_256_encode(get_password(&self.new_password) + &salt);
        let res = diesel::update(all_users.filter(users::id.eq(user_info.id)))
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
    pub fn edit_user(&self, conn: &PgConnection, user_id: Uuid) -> Result<String, String> {
        let res = diesel::update(all_users.filter(users::id.eq(user_id)))
            .set((
                users::nickname.eq(self.nickname.clone()),
                users::say.eq(self.say.clone()),
                users::email.eq(self.email.clone()),
            ))
            .get_result::<Users>(conn);
        match res {
            Ok(user) => {
                let token = Token::new(&user.into_user_info()).encode();
                match token {
                    Ok(t) => Ok(t),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl FormDataExtractor for EditUser {
    type Data = String;

    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &crate::AppState,
    ) -> Result<(Self::Data), String> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is permitted
                if !t.is_login() {
                    return Err("Permission denied, please login and try again!".to_string());
                }

                match t.user_info {
                    Some(u) => {
                        let pg_pool = &state.db.connection();
                        let user_id = u.id.clone();
                        match self.edit_user(pg_pool, user_id) {
                            Ok(token) => Ok(token),
                            Err(err) => Err(err),
                        }
                    }
                    None => Err("Permission denied, please login and retry!".to_string()),
                }
            }
            None => Err("Permission denied, please login and try again!".to_string()),
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
                    // let r = session.set("login_time", Local::now().timestamp());
                    // if r.is_err() {
                    //     error!("set session value 'login_time' failed!");
                    // }
                    // let r = session.set("info", json!(user_info).to_string());
                    // if r.is_err() {
                    //     error!("set session value 'login_time' failed!");
                    // }
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

/// Represents various user types
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum UserType {
    // Administrator
    Admin,
    // Common registered user
    Registered,
    // Visitor
    Visitor,
}

impl UserType {
    pub fn from_token(token: Option<&Token>) -> Self {
        match token {
            Some(t) => {
                if t.user_type == 0 {
                    Self::Admin
                } else {
                    Self::Registered
                }
            }
            None => Self::Visitor,
        }
    }
}
