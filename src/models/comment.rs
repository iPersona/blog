use super::super::comments;
use super::super::comments::dsl::comments as all_comments;

use super::super::UserInfo;
use super::FormDataExtractor;
use crate::models::token::TokenExtension;
use crate::{ArticlesWithTag, UserNotify};
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::sql_types::Text;
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
        let raw_sql = format!("select a.id, a.comment, a.article_id, a.user_id, b.nickname, a.create_time from comments a join users b on a.user_id=b.id where a.article_id='{}' order by a.create_time desc limit {} offset {};", id, limit, offset);
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
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "comments"]
struct InsertComments {
    comment: String,
    article_id: Uuid,
    user_id: Uuid,
}

impl InsertComments {
    fn insert(self, conn: &PgConnection) -> bool {
        diesel::insert_into(comments::table)
            .values(&self)
            .execute(conn)
            .is_ok()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewComments {
    comment: String,
    article_id: Uuid,
    reply_user_id: Option<Uuid>,
}

impl NewComments {
    fn into_insert_comments(&self, user_id: Uuid) -> InsertComments {
        InsertComments {
            comment: self.comment.clone(),
            article_id: self.article_id,
            user_id,
        }
    }

    pub fn insert(&self, conn: &PgConnection, user_info: &UserInfo) -> bool {
        self.into_insert_comments(user_info.id).insert(conn)
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
}

impl FormDataExtractor for NewComments {
    type Data = ();

    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &crate::AppState,
    ) -> Result<Self::Data, String> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                if !t.is_login() {
                    return Err("please login and try again!".to_string());
                }

                match t.user_info {
                    Some(user) => {
                        let redis_pool = &state.cache.into_inner();
                        let pg_pool = &state.db.connection();

                        let admin = UserInfo::view_admin(pg_pool, redis_pool);
                        let article = ArticlesWithTag::query_without_article(
                            &state,
                            self.article_id(),
                            false,
                        )
                        .unwrap();
                        let reply_user_id = self.reply_user_id();
                        match reply_user_id {
                            // Reply comment
                            Some(reply_user_id) => {
                                // Notification resolve
                                let user_reply_notify = UserNotify {
                                    user_id: reply_user_id,
                                    send_user_name: user.nickname.clone(),
                                    article_id: article.id,
                                    article_title: article.title.clone(),
                                    notify_type: "reply".into(),
                                };
                                user_reply_notify.cache(&redis_pool);

                                // If the sender is not an admin and also the responder is also not admin, notify admin
                                if reply_user_id != admin.id && user.groups != 0 {
                                    let comment_notify = UserNotify {
                                        user_id: admin.id,
                                        send_user_name: user.nickname.clone(),
                                        article_id: article.id,
                                        article_title: article.title.clone(),
                                        notify_type: "comment".into(),
                                    };
                                    comment_notify.cache(&redis_pool);
                                }
                            }
                            // Normal comment
                            None => {
                                if user.groups != 0 {
                                    let comment_notify = UserNotify {
                                        user_id: admin.id,
                                        send_user_name: user.nickname.clone(),
                                        article_id: article.id,
                                        article_title: article.title.clone(),
                                        notify_type: "comment".into(),
                                    };
                                    comment_notify.cache(&redis_pool);
                                }
                            }
                        }

                        let res = self.insert(&pg_pool, &user);
                        if res {
                            Ok(())
                        } else {
                            Err("new_comment failed!".to_string())
                        }
                    }
                    None => Err("Permission denied, you need to login first!".to_string()),
                }
            }
            None => Err("failed to get token extension from request!".to_string()),
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
    ) -> Result<Self::Data, String> {
        let token_ext = TokenExtension::from_request(&req);
        match token_ext {
            Some(t) => {
                // Only login user is permitted to access this API
                if !t.is_login() {
                    return Err("Permission denied, please login and try again!".to_string());
                }

                match t.user_info {
                    Some(user) => {
                        // Only the comment creator and administrator are permitted to delete comment
                        if !(user.id == self.user_id || user.is_admin()) {
                            return Err(
                                "Permission denied, you are not permitted to delete this comment!"
                                    .to_string(),
                            );
                        }

                        let pg_pool = &state.db.connection();
                        let res = self.delete(pg_pool, &user);
                        if res {
                            Ok(())
                        } else {
                            Err("failed to delete comment!".to_string())
                        }
                    }
                    None => Err("permission denied, please login and try again!".to_string()),
                }
            }
            None => Err("permission denied, please login and try again!".to_string()),
        }
    }
}
