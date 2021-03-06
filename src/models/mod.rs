pub mod article_tag_relation;
pub mod articles;
pub mod comment;
pub mod notifys;
pub mod tag;
pub mod user;
pub mod token;

pub(crate) use self::article_tag_relation::{RelationTag, Relations};
pub(crate) use self::articles::PublishedStatistics;
pub(crate) use self::articles::{ArticleList, ArticlesWithTag, EditArticle, ModifyPublish,
                                NewArticle};
pub(crate) use self::comment::{Comments, DeleteComment, NewComments};
pub(crate) use self::notifys::UserNotify;
pub(crate) use self::tag::{NewTag, TagCount, Tags};
pub(crate) use self::user::{ChangePassword, ChangePermission, DisabledUser, EditUser, LoginUser,
                            RegisteredUser, UserInfo, Users};
use std::fmt;
use actix::MailboxError;

// InnerError
#[derive(Debug)]
pub enum InnerError {
    MailboxError(MailboxError),
    ProcessError(String),
}

impl fmt::Display for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InnerError::MailboxError(e) => e.fmt(f),
            InnerError::ProcessError(e) => write!(f, "{:?}", e),
        }
    }
}
