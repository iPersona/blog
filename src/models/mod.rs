#![macro_use]

/// in used to process with extracted form data
pub trait FormDataExtractor {
    type Data;
    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &crate::AppState,
    ) -> InternalStdResult<Self::Data>;
}
pub mod article_tag_relation;
pub mod articles;
pub mod comment;
pub mod daily_statistic;
pub mod mailbox;
pub mod notifys;
pub mod tag;
pub mod token;
pub mod user;

pub(crate) use self::article_tag_relation::{RelationTag, Relations};
// pub(crate) use self::articles::PublishedStatistics;
pub(crate) use self::articles::{
    ArticleList, ArticlesWithTag, EditArticle, /*ModifyPublish,*/ NewArticle,
};
pub(crate) use self::comment::{Comments, DeleteComment, NewComments};
pub(crate) use self::notifys::UserNotify;
pub(crate) use self::tag::{NewTag, TagCount, Tags};
pub(crate) use self::user::{ChangePassword, EditUser, RegisteredUser, UserInfo};
use crate::util::result::InternalStdResult;

//use actix::MailboxError;
//use std::fmt;
//use std::error;

// InnerError
//#[derive(Debug)]
//pub enum InnerError {
//    MailboxError(MailboxError),
//    ProcessError(String),
//    ParseError(String),
//}
//
//impl error::Error for InnerError {}
//
//impl fmt::Display for InnerError {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match self {
//            InnerError::MailboxError(e) => write!(f, "{:?}", e),
//            InnerError::ProcessError(e) => write!(f, "{:?}", e),
//            InnerError::ParseError(e) => write!(f, "{:?}", e),
//        }
//    }
//}
