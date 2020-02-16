use super::super::comments;
use super::super::comments::dsl::comments as all_comments;

use super::super::UserInfo;
use super::FormDataExtractor;
use crate::models::token::TokenExtension;
use crate::util::errors::{Error, ErrorCode};
use crate::util::result::InternalStdResult;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::sql_types::Text;
use regex::Regex;
use uuid::Uuid;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "comments"]
pub struct Comments {
    id: Uuid,
    comment: String,
    article_id: Uuid,
    user_id: Uuid,
    #[sql_type = "Text"]
    nickname: String,
    create_time: NaiveDateTime,
}

impl Comments {
    pub fn query(
        conn: &PgConnection,
        limit: i64,
        offset: i64,
        id: Uuid,
    ) -> Result<Vec<Self>, String> {
        let raw_sql = format!(
            "select a.id, a.comment, a.article_id, a.user_id, b.nickname, a.create_time \
            from comments a join users b on a.user_id=b.id \
            where a.article_id='{}' \
            order by a.create_time desc \
            limit {} offset {};",
            id, limit, offset
        );
        let res = diesel::sql_query(raw_sql).get_results::<Self>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn query_with_user(
        conn: &PgConnection,
        limit: i64,
        offset: i64,
        article_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<Self>, String> {
        let raw_sql = format!(
            "select a.id, a.comment, a.article_id, a.user_id, b.nickname, a.create_time \
            from comments a join users b on a.user_id=b.id \
            where a.article_id ='{}' \
            and (a.user_id = '{}' or '{}' = any(a.mentioned_users)) \
            order by a.create_time desc \
            limit {} offset {};",
            article_id, user_id, user_id, limit, offset
        );
        let res = diesel::sql_query(raw_sql).get_results::<Self>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn delete_with_comment_id(conn: &PgConnection, id: Uuid) -> bool {
        diesel::delete(all_comments.filter(comments::id.eq(id)))
            .execute(conn)
            .is_ok()
    }

    pub fn delete_with_user_id(conn: &PgConnection, id: Uuid) -> bool {
        diesel::delete(all_comments.filter(comments::user_id.eq(id)))
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
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "comments"]
struct InsertComments {
    comment: String,
    article_id: Uuid,
    user_id: Uuid,
    mentioned_users: Option<Vec<Uuid>>,
}

impl InsertComments {
    fn insert(self, conn: &PgConnection) -> InternalStdResult<Uuid> {
        let res = diesel::insert_into(comments::table)
            .values(&self)
            .returning(comments::id)
            .get_result(conn);
        match res {
            Ok(id) => Ok(id),
            Err(e) => Err(Error {
                code: ErrorCode::DbError,
                detail: format!("failed to insert comment: {:?}", e),
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewComments {
    comment: String,
    article_id: Uuid,
    reply_user_id: Option<Uuid>,
}

impl NewComments {
    fn convert_to_insert_comments(&self, user_id: Uuid) -> InternalStdResult<InsertComments> {
        match Self::parse_mentioned_users(&user_id, self.comment.as_str()) {
            Ok(users) => Ok(InsertComments {
                comment: self.comment.clone(),
                article_id: self.article_id,
                user_id,
                mentioned_users: users,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn insert(&self, conn: &PgConnection, user_info: &UserInfo) -> InternalStdResult<Uuid> {
        self.convert_to_insert_comments(user_info.id)?.insert(conn)
    }

    pub fn reply_user_id(&self) -> Option<Uuid> {
        match self.reply_user_id {
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
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error {
                            code: ErrorCode::DbError,
                            detail: format!("new_comment failed!"),
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
