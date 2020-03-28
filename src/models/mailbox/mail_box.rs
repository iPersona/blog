use crate::comment_notify;
use crate::comment_notify::dsl::comment_notify as all_comment_notify;
use crate::util::{
    errors::{Error, ErrorCode},
    result::InternalStdResult,
};
use diesel::prelude::*;
use diesel::sql_types::Json;
use diesel::PgConnection;
use uuid::Uuid;

#[derive(Insertable, Debug, Clone, Deserialize, Serialize)]
#[table_name = "comment_notify"]
pub struct NewCommentNotify {
    pub user_id: Uuid,
    pub comment_id: Uuid,
}

impl NewCommentNotify {
    pub fn save_all(conn: &PgConnection, notifies: Vec<Self>) -> InternalStdResult<()> {
        match diesel::insert_into(comment_notify::table)
            .values(&notifies)
            .execute(conn)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error {
                code: ErrorCode::DbError,
                detail: format!("failed to insert into mail_box: {:?}", e),
            }),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "comment_notify"]
pub struct CommentNotify {
    #[sql_type = "Json"]
    pub data: serde_json::Value,
}

impl CommentNotify {
    pub fn user_notifies(
        user_id: Uuid,
        conn: &PgConnection,
    ) -> InternalStdResult<Vec<serde_json::Value>> {
        let raw_sql = format!(
            "select get_comment_notifications('{}', {}, {}) as data;",
            user_id, 10, 20
        );
        debug!("raw_sql: {}", raw_sql.as_str());
        diesel::sql_query(raw_sql)
            .get_results::<Self>(conn)
            .map_err(|e| Error {
                code: ErrorCode::DbError,
                detail: format!("failed to get user comment notifies: {:?}", e),
            })
            .map(|res| res.into_iter().map(|i| i.data).collect())
    }

    pub fn count(user_id: Uuid, conn: &PgConnection, is_unread: bool) -> InternalStdResult<i64> {
        let res = if is_unread {
            all_comment_notify
                .select(diesel::dsl::count(comment_notify::id))
                .filter(comment_notify::user_id.eq(user_id))
                .filter(comment_notify::is_read.eq(false))
                .first(conn)
        } else {
            all_comment_notify
                .select(diesel::dsl::count(comment_notify::id))
                .filter(comment_notify::user_id.eq(user_id))
                .first(conn)
        };
        match res {
            Ok(n) => Ok(n),
            Err(e) => Err(Error {
                code: ErrorCode::DbError,
                detail: format!("failed to query comment notification number: {:?}", e),
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentNotifyParam {
    pub user_id: Uuid,
}
