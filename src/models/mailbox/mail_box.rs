use crate::{
    comment_notify,
    comment_notify::dsl::comment_notify as all_comment_notify,
    util::{
        errors::{Error, ErrorCode},
        result::InternalStdResult,
    },
};
use diesel::prelude::*;
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
    pub id: i32,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub is_read: bool,
}

impl CommentNotify {
    pub fn user_notifies(user_id: Uuid, conn: &PgConnection) -> InternalStdResult<Vec<Self>> {
        all_comment_notify
            .filter(comment_notify::user_id.eq(user_id))
            .load::<Self>(conn)
            .map_err(|e| Error {
                code: ErrorCode::DbError,
                detail: format!("failed to insert into mail_box: {:?}", e),
            })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentNotifyParam {
    pub user_id: Uuid,
}
