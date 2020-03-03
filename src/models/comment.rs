use super::super::comments;
use super::super::comments::dsl::comments as all_comments;

use super::super::UserInfo;
use super::{mailbox::mail_box::NewCommentNotify, user::UserSettings, FormDataExtractor};
use crate::models::token::TokenExtension;
use crate::util::errors::{Error, ErrorCode};
use crate::{util::result::InternalStdResult, AppState};
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Json, Nullable, Text};
use regex::Regex;
use uuid::Uuid;

/// Represent comment query result
#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "comments"]
pub struct CommentResult {
    id: Uuid,
    comment: String,
    article_id: Uuid,
    from_user: Uuid,
    #[sql_type = "Text"]
    nickname: String,
    create_time: NaiveDateTime,
    to_user: Option<Uuid>,
    #[sql_type = "BigInt"]
    total: i64,
    #[sql_type = "Nullable<BigInt>"]
    sub_comments_num: Option<i64>,
}

/// Represent comment query response data of request
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentSlice<T> {
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<T>>,
}

impl<T> CommentSlice<T>
where
    T: Clone,
{
    pub fn from_comment_results<U>(comment_results: Vec<U>, total: i64) -> Self
    where
        U: Clone + Into<T>,
    {
        let comments = if comment_results.is_empty() {
            None
        } else {
            Some(comment_results.into_iter().map(|c| c.into()).collect())
        };
        CommentSlice { total, comments }
    }
}

/// Represent comments of the response data
#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "comments"]
pub struct Comments {
    id: Uuid,
    comment: String,
    article_id: Uuid,
    from_user: Uuid,
    #[sql_type = "Text"]
    nickname: String,
    create_time: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_user: Option<Uuid>,
    #[sql_type = "Nullable<BigInt>"]
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_comments_num: Option<i64>,
}

impl From<CommentResult> for Comments {
    fn from(c: CommentResult) -> Self {
        Self {
            id: c.id,
            comment: c.comment,
            article_id: c.article_id,
            from_user: c.from_user,
            nickname: c.nickname,
            create_time: c.create_time,
            to_user: c.to_user,
            sub_comments_num: c.sub_comments_num,
        }
    }
}

impl Comments {
    pub fn first_class_comments(
        conn: &PgConnection,
        limit: i64,
        offset: i64,
        article_id: Uuid,
    ) -> Result<CommentSlice<Self>, String> {
        // TODO: 使用row_to_json和array_to_json来重写sql，免去后续用rust再处理一遍返回数据
        // reference:
        // [row_to_json & array_to_josn](https://hashrocket.com/blog/posts/faster-json-generation-with-postgresql)
        // [如何将行数据转换成array](https://www.postgresqltutorial.com/postgresql-aggregate-functions/postgresql-array_agg-function/)
        let raw_sql = format!(
            "with sub_comments as (
                select distinct parent_comment, count(*) over(partition by parent_comment) as sub_comments_num
                from comments
                where article_id='{}'
                and parent_comment is not null
            ),
            parent_comments as (
                select a.id, a.comment, a.article_id, a.from_user, b.nickname, a.create_time, a.to_user, count(*) over() as total
                from comments a join users b on a.from_user=b.id
                where a.article_id='{}'
                and a.parent_comment is null
            )
            select *
            from parent_comments p left join sub_comments s on p.id=s.parent_comment
            order by p.create_time desc
            limit {} offset {};",
            article_id, article_id, limit, offset
        );
        let res = diesel::sql_query(raw_sql).get_results::<CommentResult>(conn);
        match res {
            Ok(data) => {
                let total = if data.is_empty() { 0 } else { data[0].total };
                Ok(CommentSlice::from_comment_results(data, total))
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn user_comments(
        conn: &PgConnection,
        limit: i64,
        offset: i64,
        article_id: Uuid,
        user_id: Uuid,
    ) -> Result<CommentSlice<Self>, String> {
        let raw_sql = format!(
            "select a.id, a.comment, a.article_id, a.from_user, b.nickname, a.create_time, a.to_user, count(*) over() as total \
            from comments a join users b on a.from_user=b.id \
            where a.article_id ='{}' \
            and (a.from_user = '{}' or '{}' = any(a.mentioned_users)) \
            order by a.create_time desc \
            limit {} offset {};",
            article_id, user_id, user_id, limit, offset
        );
        let res = diesel::sql_query(raw_sql).get_results::<CommentResult>(conn);
        match res {
            Ok(data) => {
                let total = if data.is_empty() { 0 } else { data[0].total };
                Ok(CommentSlice::from_comment_results(data, total))
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn delete_with_comment_id(conn: &PgConnection, id: Uuid) -> bool {
        diesel::delete(all_comments.filter(comments::id.eq(id)))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_with_user_id(conn: &PgConnection, id: Uuid) -> bool {
        diesel::delete(all_comments.filter(comments::from_user.eq(id)))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_with_article_id(conn: &PgConnection, id: Uuid) -> bool {
        diesel::delete(all_comments.filter(comments::article_id.eq(id)))
            .execute(conn)
            .is_ok()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentQueryOption {
    pub limit: i64,
    pub offset: i64,
    pub user_id: Option<Uuid>,
    pub parent_comment: Option<Uuid>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "comments"]
pub struct SubComment {
    id: Uuid,
    comment: String,
    article_id: Uuid,
    from_user: Uuid,
    #[sql_type = "Text"]
    from_nickname: String,
    create_time: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_user: Option<Uuid>,
    #[sql_type = "Nullable<Text>"]
    to_nickname: Option<String>,
}

impl SubComment {
    pub fn comments(
        conn: &PgConnection,
        limit: i64,
        offset: i64,
        article_id: Uuid,
        parent_comment_id: Uuid,
    ) -> Result<CommentSlice<Self>, String> {
        let raw_sql = format!(
            "with from_user as (    \
                select a.id, a.comment, a.article_id, a.from_user, b.nickname, a.create_time, a.to_user \
                from comments a join users b on a.from_user=b.id \
                where a.article_id='{}' \
                    and a.parent_comment='{}' \
            ) \
            select a.id, a.comment, a.article_id, a.from_user, a.nickname as from_nickname, a.create_time, a.to_user, b.nickname as to_nickname \
                from from_user a left join users b on a.to_user=b.id \
                order by a.create_time desc \
                limit {} offset {};",
            article_id, parent_comment_id, limit, offset
        );
        let res = diesel::sql_query(raw_sql).get_results::<Self>(conn);
        match res {
            Ok(data) => {
                let total = if data.is_empty() { 0 } else { data.len() };
                Ok(CommentSlice::<Self> {
                    total: total as i64,
                    comments: Some(data),
                })
            }
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "comments"]
struct InsertComments {
    comment: String,
    article_id: Uuid,
    from_user: Uuid,
    to_user: Option<Uuid>,
    parent_comment: Option<Uuid>,
    mentioned_users: Option<Vec<Uuid>>,
}

impl InsertComments {
    fn insert(self, conn: &PgConnection) -> InternalStdResult<CommentEntity> {
        let res = diesel::insert_into(comments::table)
            .values(&self)
            // .returning(comments::id)
            .get_result(conn);
        match res {
            Ok(c) => Ok(c),
            Err(e) => Err(Error {
                code: ErrorCode::DbError,
                detail: format!("failed to insert comment: {:?}", e),
            }),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "comments"]
pub struct CommentEntity {
    pub id: Uuid,
    pub comment: String,
    pub article_id: Uuid,
    pub from_user: Uuid,
    pub create_time: NaiveDateTime,
    pub mentioned_users: Option<Vec<Uuid>>,
    pub to_user: Option<Uuid>,
    pub parent_comment: Option<Uuid>,
}

impl CommentEntity {
    pub fn notified_users(&self) -> Option<Vec<Uuid>> {
        let mut users = Vec::new();
        if let Some(u) = self.to_user {
            users.push(u);
        }
        if let Some(u) = &self.mentioned_users {
            users.extend_from_slice(u.as_slice())
        }
        if users.is_empty() {
            None
        } else {
            Some(users)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewComments {
    comment: String,
    article_id: Uuid,
    to_user: Option<Uuid>,
    parent_comment: Option<Uuid>,
}

impl NewComments {
    fn convert_to_insert_comments(&self, user_id: Uuid) -> InternalStdResult<InsertComments> {
        match Self::parse_mentioned_users(&user_id, self.comment.as_str()) {
            Ok(users) => Ok(InsertComments {
                comment: self.comment.clone(),
                article_id: self.article_id,
                from_user: user_id,
                to_user: self.to_user,
                mentioned_users: users,
                parent_comment: self.parent_comment,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn insert(
        &self,
        conn: &PgConnection,
        user_info: &UserInfo,
    ) -> InternalStdResult<CommentEntity> {
        if self.comment.is_empty() {
            return Err(Error {
                code: ErrorCode::InvalidContent,
                detail: format!("comment content can not be empty!"),
            });
        }

        self.convert_to_insert_comments(user_info.id)?.insert(conn)
    }

    pub fn reply_user_id(&self) -> Option<Uuid> {
        match self.to_user {
            Some(id) => Some(id.clone()),
            None => None,
        }
    }

    pub fn article_id(&self) -> Uuid {
        self.article_id
    }

    pub fn parse_mentioned_users(
        user_id: &Uuid,
        content: &str,
    ) -> InternalStdResult<Option<Vec<Uuid>>> {
        // since rust regex crate does not support `look-around`, we need to remove blockquote lines first
        // to avoid re-including in mentioned users
        //        let mut non_quote_content = self.comment.clone();
        //        non_quote_content.lines().into_iter()
        //            .filter(|&line| !line.contains("> "))
        //            .map(|line| {
        //                self.parse_uuid(line)
        //            })
        //            // merge uuid arrays
        //            .flat_map(|users| {
        //                match users {
        //                    Ok(v) => if v.is_empty() {
        //                        Ok(None)
        //                    } else {
        //                        Ok(Some(v))
        //                    },
        //                    Err(e) => Err(e),
        //                }
        //            })
        //            .collect::<InternalStdResult<Option<Vec<Uuid>>>>()

        let mut non_quote_content = String::from(content);
        non_quote_content = non_quote_content
            .lines()
            .into_iter()
            .filter(|&line| !line.contains("> "))
            .collect::<Vec<&str>>()
            .join("");
        let res = Self::parse_uuid(user_id, non_quote_content.as_str());
        match res {
            Ok(v) => {
                if v.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(v))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn parse_uuid(user_id: &Uuid, content: &str) -> InternalStdResult<Vec<Uuid>> {
        let mut users: Vec<Uuid> = Vec::new();
        let re = Regex::new(r#"]\(/#/user/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})\)"#).unwrap();
        for cap in re.captures_iter(content) {
            match Uuid::parse_str(cap[1].trim()) {
                Ok(id) => {
                    // Not owner of this comment and current list doesn't contain the user
                    if !id.eq(user_id) && !users.contains(&id) {
                        users.push(id.clone())
                    }
                }
                Err(e) => {
                    return Err(Error {
                        code: ErrorCode::ParseError,
                        detail: format!("failed to parse uuid in comment: {:?}", e),
                    })
                }
            }
        }
        Ok(users)
    }

    pub fn send_to_mailbox(&self, state: &AppState, comment: &CommentEntity) {
        let conn = &state.db.connection();
        let redis_pool = &state.cache.into_inner();
        let users = comment.notified_users();
        if let Some(users) = users {
            let mails = users
                .into_iter()
                .filter(|u| {
                    let user_settings = UserSettings::get(redis_pool, u).unwrap();
                    user_settings.settings.subscribe
                })
                .map(|u| NewCommentNotify {
                    user_id: u,
                    comment_id: comment.id,
                })
                .collect::<Vec<NewCommentNotify>>();
            if let Err(e) = NewCommentNotify::save_all(conn, mails) {
                error!("failed to send comment to mailbox: {:?}", e)
            }

            // TODO: send notify email
        }
    }
}

impl FormDataExtractor for NewComments {
    type Data = ();

    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &crate::AppState,
    ) -> InternalStdResult<Self::Data> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                if !t.is_login() {
                    return Err(Error {
                        code: ErrorCode::PermissionDenied,
                        detail: format!("please login and try again!"),
                    });
                }
                let conn = &state.db.connection();
                match t.user_info {
                    Some(user) => match self.insert(&conn, &user) {
                        Ok(c) => {
                            self.send_to_mailbox(state, &c);
                            Ok(())
                        }
                        Err(e) => Err(Error {
                            code: e.code,
                            detail: format!("new_comment failed: {:?}", e.detail),
                        }),
                    },
                    None => Err(Error {
                        code: ErrorCode::PermissionDenied,
                        detail: format!("failed to get token extension from request!"),
                    }),
                }
            }
            None => Err(Error {
                code: ErrorCode::PermissionDenied,
                detail: format!("failed to get token extension from request!"),
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteComment {
    comment_id: Uuid,
    user_id: Uuid,
}

impl DeleteComment {
    pub fn delete(&self, conn: &PgConnection, user_info: &UserInfo) -> bool {
        if user_info.is_admin() {
            return Comments::delete_with_comment_id(conn, self.comment_id);
        }
        if self.user_id == user_info.id {
            Comments::delete_with_comment_id(conn, self.comment_id)
        } else {
            false
        }
    }
}

impl FormDataExtractor for DeleteComment {
    type Data = ();

    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &crate::AppState,
    ) -> InternalStdResult<Self::Data> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is permitted to access this API
                if !t.is_login() {
                    return Err(Error {
                        code: ErrorCode::PermissionDenied,
                        detail: format!("Permission denied, please login and try again!"),
                    });
                }

                match t.user_info {
                    Some(user) => {
                        // Only the comment creator and administrator are permitted to delete comment
                        if !(user.id == self.user_id || user.is_admin()) {
                            return Err(
                                Error {
                                    code: ErrorCode::PermissionDenied,
                                    detail: format!("Permission denied, you are not permitted to delete this comment!")
                                }
                            );
                        }

                        let pg_pool = &state.db.connection();
                        let res = self.delete(pg_pool, &user);
                        if res {
                            Ok(())
                        } else {
                            Err(Error {
                                code: ErrorCode::DbError,
                                detail: format!("failed to delete comment!"),
                            })
                        }
                    }
                    None => Err(Error {
                        code: ErrorCode::PermissionDenied,
                        detail: format!("permission denied, please login and try again!"),
                    }),
                }
            }
            None => Err(Error {
                code: ErrorCode::PermissionDenied,
                detail: format!("permission denied, please login and try again!"),
            }),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
pub struct CommentLocation {
    #[sql_type = "Json"]
    pub data: serde_json::Value,
}

impl CommentLocation {
    pub fn locate(conn: &PgConnection, args: &CommentLocationArgs) -> InternalStdResult<Self> {
        let raw_sql = format!(
            "select comment_data('{}', '{}', {}) as data;",
            args.article_id.to_hyphenated().to_string(),
            args.comment_id.to_hyphenated().to_string(),
            args.page_size
        );
        diesel::sql_query(raw_sql)
            .get_result::<Self>(conn)
            .map_err(|e| Error {
                code: ErrorCode::DbError,
                detail: format!("failed to load comment location data: {:?}", e),
            })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentLocationArgs {
    pub article_id: Uuid,
    pub comment_id: Uuid,
    pub page_size: i64,
}
