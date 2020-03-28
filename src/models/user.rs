use super::super::users;
use super::super::users::dsl::users as all_users;

use super::FormDataExtractor;
use super::UserNotify;
use crate::util::errors;
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use serde_json;
use std::sync::Arc;
use uuid::Uuid;

use super::super::{
    /*get_github_primary_email, */ get_password, random_string, sha3_256_encode, RedisPool,
};
use super::{mailbox::mail_box::CommentNotify, token::Token};
use crate::models::token::TokenExtension;
use crate::util::env::Env;
use crate::util::errors::{Error, ErrorCode};
use crate::util::redis_pool::RedisKeys;
use crate::util::result::InternalStdResult;
use crate::AppState;
use std::cell::Ref;
use std::{collections::HashMap, convert::TryFrom};
use time::Duration;

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
    pub is_active: bool,
    // Subscribe to comment with email notification
    pub subscribe: bool,
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

    pub fn user(conn: &PgConnection, id: &Uuid) -> Option<Self> {
        let res = all_users.filter(users::id.eq(id)).first::<Users>(conn);
        match res {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }

    pub fn active_account(conn: &PgConnection, user_id: &Uuid) -> bool {
        let res = diesel::update(all_users.filter(users::id.eq(user_id)))
            .set(users::is_active.eq(true))
            .execute(conn);
        match res {
            Ok(data) => {
                if data == 1 {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn clear_unverified(conn: &PgConnection) -> InternalStdResult<()> {
        let expect_create_time =
            (Utc::now() - Duration::hours(Env::get().verify_token_expired)).naive_utc();
        let res = diesel::delete(
            all_users
                .filter(users::is_active.eq(false))
                .filter(users::create_time.lt(expect_create_time)),
        )
        .execute(conn);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(Error {
                code: ErrorCode::Unknown,
                detail: format!("failed to delete unverified user: {}", err),
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSettingVal {
    pub subscribe: bool,
}

impl From<UserSettingResult> for UserSettingVal {
    fn from(val: UserSettingResult) -> Self {
        Self {
            subscribe: val.subscribe,
        }
    }
}

impl TryFrom<String> for UserSettingVal {
    type Error = crate::util::errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(value.as_str()).map_err(|e| Error {
            code: ErrorCode::ParseError,
            detail: format!("parse UserSettings from string failed: {:?}", e),
        })
    }
}

impl UserSettingVal {
    pub fn to_string(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "users"]
pub struct UserSettingResult {
    pub id: Uuid,
    pub subscribe: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSettings {
    pub id: Uuid,
    pub settings: UserSettingVal,
}

impl UserSettings {
    pub fn load(conn: &PgConnection, redis_pool: &Arc<RedisPool>) -> InternalStdResult<()> {
        // get user settings
        let res = all_users
            .select((users::id, users::subscribe))
            .get_results::<UserSettingResult>(conn);
        match res {
            Ok(user_settings) => {
                let redis_key = RedisKeys::Users.to_string();
                // delete old data
                redis_pool.del(redis_key.as_str());

                // load new data
                let args = user_settings
                    .into_iter()
                    .map(|s| {
                        Self {
                            id: s.id,
                            settings: UserSettingVal {
                                subscribe: s.subscribe,
                            },
                        }
                        .to_array()
                    })
                    .flatten()
                    .collect::<Vec<String>>();

                redis_pool.hmset(redis_key.as_str(), args);
                Ok(())
            }
            Err(e) => Err(Error {
                code: ErrorCode::DbError,
                detail: format!("failed to get user settings from db: {:?}", e),
            }),
        }
    }

    pub fn to_array(self) -> Vec<String> {
        vec![
            self.id.to_hyphenated().to_string(),
            serde_json::to_value(self.settings).unwrap().to_string(),
        ]
    }

    pub fn get(redis: &Arc<RedisPool>, user_id: &Uuid) -> InternalStdResult<Self> {
        let key = RedisPool::get_redis_key(RedisKeys::Users);
        let s = redis.hget::<String>(key.as_str(), user_id.to_hyphenated().to_string().as_str())?;

        let setting_val = UserSettingVal::try_from(s)?;
        Ok(UserSettings {
            id: user_id.clone(),
            settings: setting_val,
        })
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

    fn insert(&self, state: &AppState) -> Result<Users, String> {
        let conn = state.db.connection();
        match diesel::insert_into(users::table)
            .values(self)
            .get_result::<Users>(&conn)
        {
            Ok(user) => Ok(user),
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
    pub fn insert(self, state: &AppState) -> Result<Users, String> {
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
    pub fn from_str(s: &str) -> InternalStdResult<Self> {
        serde_json::from_str::<Self>(s).map_err(|e| Error {
            code: ErrorCode::ParseError,
            detail: format!("failed to parse UserInfo from string: {:?}", e),
        })
    }

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

    /// Get admin information, cron on redis
    /// key is `admin_info`
    pub fn view_admin(conn: &PgConnection, redis_pool: &Arc<RedisPool>) -> Self {
        if redis_pool.exists("admin_info") {
            UserInfo::from_str(&redis_pool.get("admin_info").unwrap()).unwrap()
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
                let token = Token::new(&user.into_user_info(), true).encode();
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
    ) -> InternalStdResult<Self::Data> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is permitted
                if !t.is_login() {
                    return Err(Error {
                        code: ErrorCode::PermissionDenied,
                        detail: format!("Permission denied, please login and try again!"),
                    });
                }

                match t.user_info {
                    Some(u) => {
                        let pg_pool = &state.db.connection();
                        let user_id = u.id.clone();
                        match self.edit_user(pg_pool, user_id) {
                            Ok(token) => Ok(token),
                            Err(err) => Err(Error {
                                code: ErrorCode::DbError,
                                detail: format!("failed to edit user info: {:?}", err),
                            }),
                        }
                    }
                    None => Err(Error {
                        code: ErrorCode::PermissionDenied,
                        detail: format!("Permission denied, please login and retry!"),
                    }),
                }
            }
            None => Err(Error {
                code: ErrorCode::PermissionDenied,
                detail: format!("Permission denied, please login and try again!"),
            }),
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
    ) -> Result<UserInfo, errors::Error> {
        let res = all_users
            .filter(users::disabled.eq(0))
            .filter(users::account.eq(self.account.to_owned()))
            .get_result::<Users>(conn);
        match res {
            Ok(data) => {
                // check whether user email is verified
                if !data.is_active {
                    return Err(errors::Error {
                        code: errors::ErrorCode::EmailNotVerified,
                        detail: "Email is not verified!".to_string(),
                    });
                }

                if data.password == sha3_256_encode(get_password(&self.password) + &data.salt) {
                    let user_info = data.into_user_info();
                    Ok(user_info)
                } else {
                    Err(errors::Error {
                        code: errors::ErrorCode::LoginFailed,
                        detail: "Invalid user name or password!".to_string(),
                    })
                }
            }
            Err(err) => Err(errors::Error {
                code: errors::ErrorCode::Unknown,
                detail: format!("{}", err),
            }),
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Verify {
    pub token: String,
}

/// Represent login data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginData {
    pub token: Option<String>,
    pub notify_num: Option<i64>,
}

pub struct LoginDataBuilder {
    data: LoginData,
}

impl LoginDataBuilder {
    pub fn new() -> Self {
        LoginDataBuilder {
            data: LoginData {
                token: None,
                notify_num: None,
            },
        }
    }

    pub fn build(self) -> LoginData {
        self.data
    }

    pub fn token(mut self, t: Token) -> InternalStdResult<Self> {
        match t.encode() {
            Ok(token) => self.data.token = Some(token),
            Err(e) => return Err(e),
        };
        Ok(self)
    }

    fn notify_num(mut self, nty_num: i64) -> InternalStdResult<Self> {
        self.data.notify_num = Some(nty_num);
        Ok(self)
    }
}

pub fn login_data(conn: &PgConnection, user_info: &UserInfo) -> InternalStdResult<LoginData> {
    let token = Token::new(&user_info, true);
    let notify_num = CommentNotify::count(user_info.id, conn, true)?;
    let data = LoginDataBuilder::new()
        .token(token)?
        .notify_num(notify_num)?
        .build();
    Ok(data)
}

pub fn verify_data(conn: &PgConnection, token: Token) -> InternalStdResult<LoginData> {
    let user_id = Uuid::parse_str(token.user_id.as_str()).map_err(|e| Error {
        code: ErrorCode::ParseError,
        detail: format!("{:?}", e),
    })?;
    let notify_num = CommentNotify::count(user_id, conn, true)?;
    let data = LoginDataBuilder::new()
        .token(token)?
        .notify_num(notify_num)?
        .build();
    Ok(data)
}
