use std::collections::HashMap;
use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};
use strip_markdown::strip_markdown;
use uuid::Uuid;

use crate::{AppState, Comments};

use super::super::article_with_tag::dsl::article_with_tag as all_article_with_tag;
use super::super::articles::dsl::articles as all_articles;
use super::super::markdown_render;
use super::super::{article_with_tag, articles};
use super::FormDataExtractor;
use super::{RelationTag, Relations, UserNotify};
use crate::cron::cache::IncreaseArticleVisitNum;
use crate::models::token::TokenExtension;
use crate::util::errors::{Error, ErrorCode};
use crate::util::result::InternalStdResult;
use log::error;
use std::cell::Ref;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticlesWithTag {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub tags_id: Vec<Option<Uuid>>,
    pub tags: Vec<Option<String>>,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl ArticlesWithTag {
    pub fn delete_with_id(state: &AppState, id: Uuid) -> Result<usize, String> {
        let conn = &state.db.connection();
        let redis_pool = &state.cache.into_inner();
        // Delete from table `article_tag_relation`
        Relations::delete_all(conn, id, "article");
        // Delete from table `articles`
        let res = diesel::delete(all_articles.filter(articles::id.eq(&id))).execute(conn);
        match res {
            Ok(data) => {
                // Delete from redis cache
                UserNotify::remove_with_article(id, redis_pool);
                // Delete all comments of the article table ``
                Comments::delete_with_article_id(conn, id);
                Ok(data)
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn query_article(
        state: &AppState,
        id: Uuid,
        admin: bool,
    ) -> Result<ArticlesWithTag, String> {
        // statistic visitor number
        state.cache_worker_addr.do_send(IncreaseArticleVisitNum {
            article_id: id.clone(),
        });

        let conn = &state.db.connection();
        let res = if admin {
            all_article_with_tag
                .filter(article_with_tag::id.eq(id))
                .get_result::<RawArticlesWithTag>(conn)
        } else {
            all_article_with_tag
                .filter(article_with_tag::id.eq(id))
                .filter(article_with_tag::published.eq(true))
                .get_result::<RawArticlesWithTag>(conn)
        };
        match res {
            Ok(data) => Ok(data.into_markdown()),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn query_without_article(
        state: &AppState,
        id: Uuid,
        admin: bool,
    ) -> Result<ArticlesWithoutContent, String> {
        let conn = &state.db.connection();
        let res = if admin {
            all_article_with_tag
                .filter(article_with_tag::id.eq(id))
                .get_result::<RawArticlesWithTag>(conn)
        } else {
            all_article_with_tag
                .filter(article_with_tag::id.eq(id))
                .filter(article_with_tag::published.eq(true))
                .get_result::<RawArticlesWithTag>(conn)
        };
        match res {
            Ok(data) => Ok(data.into_without_content()),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn query_raw_article(state: &AppState, id: Uuid) -> Result<ArticlesWithTag, String> {
        let conn = &state.db.connection();
        let res = all_article_with_tag
            .filter(article_with_tag::id.eq(id))
            .get_result::<RawArticlesWithTag>(conn);
        match res {
            Ok(data) => Ok(data.into_markdown()),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn publish_article(state: &AppState, data: &ModifyPublish) -> Result<usize, String> {
        let conn = &state.db.connection();
        let res = diesel::update(all_articles.filter(articles::id.eq(data.id)))
            .set(articles::published.eq(data.publish))
            .execute(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "article_with_tag"]
pub struct ArticleSummary {
    pub id: Uuid,
    pub title: String,
    pub raw_content: String,
    pub published: bool,
    pub tags: Vec<Option<String>>,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl ArticleSummary {
    /// list all articles with tags and summary in a range
    pub fn list_articles(
        conn: &PgConnection,
        limit: i64,
        offset: i64,
        admin: bool,
    ) -> Result<Vec<Self>, String> {
        let res = if admin {
            all_article_with_tag
                .select((
                    article_with_tag::id,
                    article_with_tag::title,
                    article_with_tag::raw_content,
                    article_with_tag::published,
                    article_with_tag::tags,
                    article_with_tag::create_time,
                    article_with_tag::modify_time,
                ))
                .order(article_with_tag::create_time.desc())
                .limit(limit)
                .offset(offset)
                .load::<Self>(conn)
        } else {
            all_article_with_tag
                .select((
                    article_with_tag::id,
                    article_with_tag::title,
                    article_with_tag::raw_content,
                    article_with_tag::published,
                    article_with_tag::tags,
                    article_with_tag::create_time,
                    article_with_tag::modify_time,
                ))
                .filter(article_with_tag::published.eq(true))
                .order(article_with_tag::create_time.desc())
                .limit(limit)
                .offset(offset)
                .load::<Self>(conn)
        };
        match res {
            Ok(mut data) => {
                for d in &mut data {
                    d.summary();
                    d.trim_tags();
                }
                Ok(data)
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn summary(&mut self) {
        let summary: String = strip_markdown(self.raw_content.as_str())
            .as_str()
            .replace("\n", "") // remove newline
            .to_string()
            .chars()
            .take(50) // TODO: limit summary length
            .collect();
        self.raw_content = [summary.as_str(), "..."].concat()
    }

    fn trim_tags(&mut self) {
        if self.tags.len() == 1 && self.tags[0].is_none() {
            self.tags.clear();
        }
    }

    pub fn list_articles_with_tag(
        conn: &PgConnection,
        tag_id: Uuid,
        limit: i64,
        offset: i64,
        admin: bool,
    ) -> Result<Vec<Self>, String> {
        let raw_sql = format!("select id, title, raw_content, published, tags, create_time, modify_time from article_with_tag where ('{}' = any(tags_id)) {} order by create_time desc limit {} offset {}", tag_id, if admin {""} else {"and published = true"}, limit, offset);
        let res = diesel::sql_query(raw_sql).load::<Self>(conn);
        match res {
            Ok(mut data) => {
                for d in &mut data {
                    d.summary(); // brief content to summary
                    d.trim_tags(); // remove null item
                }
                Ok(data)
            }
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn query_article_numbers_with_tag(
        state: &AppState,
        tag_id: Uuid,
        admin: bool,
    ) -> Result<i64, String> {
        #[derive(QueryableByName)]
        struct Count {
            #[sql_type = "BigInt"]
            count: i64,
        }

        let raw_sql = format!(
            "select count(*) from article_with_tag where ('{}' = any(tags_id)) {}",
            tag_id,
            if admin { "" } else { "and published = true" }
        );
        let conn = &state.db.connection();
        let res = diesel::sql_query(raw_sql).load::<Count>(conn);
        match res {
            Ok(n) => Ok(n[0].count),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "articles"]
pub struct ArticleList {
    pub id: Uuid,
    pub title: String,
    pub published: bool,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl ArticleList {
    pub fn query_list_article(
        state: &AppState,
        limit: i64,
        offset: i64,
        admin: bool,
    ) -> Result<Vec<ArticleList>, String> {
        let conn = &state.db.connection();
        let res = if admin {
            all_articles
                .select((
                    articles::id,
                    articles::title,
                    articles::published,
                    articles::create_time,
                    articles::modify_time,
                ))
                .order(articles::create_time.desc())
                .limit(limit)
                .offset(offset)
                .load::<ArticleList>(conn)
        } else {
            all_articles
                .select((
                    articles::id,
                    articles::title,
                    articles::published,
                    articles::create_time,
                    articles::modify_time,
                ))
                .filter(articles::published.eq(true))
                .order(articles::create_time.desc())
                .limit(limit)
                .offset(offset)
                .load::<ArticleList>(conn)
        };

        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub fn query_article_numbers(state: &AppState, admin: bool) -> Result<i64, String> {
        let conn = &state.db.connection();
        let res = if admin {
            all_articles
                .select(diesel::dsl::count(articles::id))
                .filter(articles::published.eq(true))
                .first(conn)
        } else {
            all_articles
                .select(diesel::dsl::count(articles::id))
                .first(conn)
        };
        match res {
            Ok(n) => Ok(n),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "articles"]
struct InsertArticle {
    title: String,
    raw_content: String,
    content: String,
    published: bool,
}

impl InsertArticle {
    fn new(title: String, raw_content: String, published: bool) -> Self {
        let content = markdown_render(&raw_content);
        InsertArticle {
            title,
            raw_content,
            content,
            published,
        }
    }

    fn insert(&self, conn: &PgConnection) -> Articles {
        diesel::insert_into(articles::table)
            .values(self)
            .get_result::<Articles>(conn)
            .unwrap()
    }
}

#[derive(Deserialize, Serialize)]
pub struct NewArticle {
    pub title: String,
    pub raw_content: String,
    pub exist_tags: Option<Vec<Uuid>>,
    pub new_tags: Option<Vec<String>>,
    pub publish: bool,
}

impl NewArticle {
    pub fn insert(&self, conn: &PgConnection) -> bool {
        let article = self.convert_insert_article().insert(conn);
        let new_tags = match &self.new_tags {
            Some(t) => Some(t.clone()),
            None => None,
        };
        let exist_tags = match &self.exist_tags {
            Some(t) => Some(t.clone()),
            None => None,
        };
        if new_tags.is_some() || exist_tags.is_some() {
            RelationTag::new(article.id, new_tags, exist_tags).insert_all(conn)
        } else {
            true
        }
    }

    fn convert_insert_article(&self) -> InsertArticle {
        InsertArticle::new(
            self.title.to_owned(),
            self.raw_content.to_owned(),
            self.publish,
        )
    }
}

impl FormDataExtractor for NewArticle {
    type Data = ();

    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &AppState,
    ) -> InternalStdResult<Self::Data> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return Err(Error {
                code: ErrorCode::PermissionDenied,
                detail: format!("Permission denied, this API is for administrator only!"),
            });
        }

        let conn = &state.db.connection();
        let r = self.insert(conn);
        if r {
            Ok(())
        } else {
            Err(Error {
                code: ErrorCode::DbError,
                detail: format!("create article failed!"),
            })
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct EditArticle {
    id: Uuid,
    title: String,
    raw_content: String,
    new_choice_already_exists_tags: Option<Vec<Uuid>>,
    deselect_tags: Option<Vec<Uuid>>,
    new_tags: Option<Vec<String>>,
}

impl EditArticle {
    pub fn edit_article(&self, state: &AppState) -> Result<usize, String> {
        let conn = state.db.connection();
        let res = diesel::update(all_articles.filter(articles::id.eq(self.id)))
            .set((
                articles::title.eq(self.title.clone()),
                articles::content.eq(markdown_render(&self.raw_content.clone())),
                articles::raw_content.eq(self.raw_content.clone()),
            ))
            .execute(&conn);
        if self.new_tags.is_some() || self.new_choice_already_exists_tags.is_some() {
            RelationTag::new(
                self.id,
                self.new_tags.clone(),
                self.new_choice_already_exists_tags.clone(),
            )
            .insert_all(&conn);
        }
        if self.deselect_tags.is_some() {
            for i in self.deselect_tags.clone().unwrap() {
                Relations::new(self.id, i).delete_relation(&conn);
            }
        }
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

impl FormDataExtractor for EditArticle {
    type Data = ();

    fn execute(
        &self,
        req: actix_web::HttpRequest,
        state: &AppState,
    ) -> InternalStdResult<Self::Data> {
        // The API is only available for administrator
        if !TokenExtension::is_admin(&req) {
            return Err(Error {
                code: ErrorCode::PermissionDenied,
                detail: format!("Permission denied, this API is for administrator only!"),
            });
        }

        let res = self.edit_article(state);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(Error {
                code: ErrorCode::DbError,
                detail: format!("edit_article failed: {:?}", e),
            }),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ModifyPublish {
    id: Uuid,
    publish: bool,
}

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
struct RawArticlesWithTag {
    pub id: Uuid,
    pub title: String,
    pub raw_content: String,
    pub content: String,
    pub published: bool,
    pub tags_id: Vec<Option<Uuid>>,
    pub tags: Vec<Option<String>>,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

impl RawArticlesWithTag {
    fn into_markdown(self) -> ArticlesWithTag {
        ArticlesWithTag {
            id: self.id,
            title: self.title,
            content: self.raw_content, // rander on client browser
            published: self.published,
            tags_id: self.tags_id,
            tags: self.tags,
            create_time: self.create_time,
            modify_time: self.modify_time,
        }
    }

    // fn into_html(self) -> ArticlesWithTag {
    //     ArticlesWithTag {
    //         id: self.id,
    //         title: self.title,
    //         content: self.content,
    //         published: self.published,
    //         tags_id: self.tags_id,
    //         tags: self.tags,
    //         create_time: self.create_time,
    //         modify_time: self.modify_time,
    //     }
    // }

    fn into_without_content(self) -> ArticlesWithoutContent {
        ArticlesWithoutContent {
            id: self.id,
            title: self.title,
            published: self.published,
            tags_id: self.tags_id,
            tags: self.tags,
            create_time: self.create_time,
            modify_time: self.modify_time,
        }
    }
}

#[derive(Queryable, Debug, Clone)]
struct Articles {
    pub id: Uuid,
    pub title: String,
    pub raw_content: String,
    pub content: String,
    pub published: bool,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
    pub visitor_num: i64,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "articles"]
pub struct PublishedStatistics {
    #[sql_type = "Text"]
    pub dimension: String,
    #[sql_type = "BigInt"]
    pub quantity: i64,
}

impl PublishedStatistics {
    pub fn statistics_published_frequency_by_month(
        conn: &PgConnection,
    ) -> Result<Vec<PublishedStatistics>, String> {
        let raw_sql = "select to_char(create_time, 'yyyy-mm') as dimension, count(*) as quantity from articles group by dimension order by dimension;";
        let res = diesel::sql_query(raw_sql).load::<Self>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticlesWithoutContent {
    pub id: Uuid,
    pub title: String,
    pub published: bool,
    pub tags_id: Vec<Option<Uuid>>,
    pub tags: Vec<Option<String>>,
    pub create_time: NaiveDateTime,
    pub modify_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleSlice {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteArticlesWithTags {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdminViewRawArticle {
    pub id: String,
}

impl AdminViewRawArticle {
    pub fn new(query: Ref<HashMap<String, String>>) -> Option<AdminViewRawArticle> {
        query
            .get("id")
            .map_or(None, |id| Some(AdminViewRawArticle { id: (*id).clone() }))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuerySlice {
    pub limit: i64,
    pub offset: i64,
}

impl QuerySlice {
    pub fn new(query: Ref<HashMap<String, String>>) -> Option<QuerySlice> {
        let limit = query
            .get("limit")
            .map_or(-1, |limit| limit.parse::<i64>().unwrap_or_else(|_| -1));
        let offset = query
            .get("offset")
            .map_or(-1, |offset| offset.parse::<i64>().unwrap_or_else(|_| -1));
        if limit == -1 || offset == -1 {
            return None;
        }
        Some(QuerySlice { limit, offset })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentsResponse {
    pub comments: Vec<Comments>,
    pub admin: bool,
    pub user: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewArticle {
    pub id: Uuid,
}

impl ViewArticle {
    pub fn new(query: Ref<HashMap<String, String>>) -> Option<ViewArticle> {
        match query.get("id") {
            Some(v) => match Uuid::from_str(v) {
                Ok(id) => Some(ViewArticle { id }),
                Err(_) => None,
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArticleNumWithTag {
    pub tag_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListComments {
    pub article_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateArticleVisitNum {
    pub article_id: Uuid,
    pub visit_num: i64,
}

impl UpdateArticleVisitNum {
    pub fn from_strings(mut str_list: Vec<String>) -> Vec<Self> {
        let mut items = Vec::new();
        while str_list.len() > 0 {
            let num_str = str_list.pop().unwrap();
            let id_str = str_list.pop().unwrap();

            items.push(Self {
                article_id: Uuid::parse_str(id_str.as_str()).unwrap(),
                visit_num: num_str.as_str().parse::<i64>().unwrap(),
            })
        }
        items
    }
}

impl UpdateArticleVisitNum {
    pub fn update_visit_num(&self, conn: &PgConnection) -> Result<usize, String> {
        // let res = diesel::update(all_articles.filter(articles::id.eq(self.article_id.clone())))
        //     .set(articles::visitor_num.eq(self.visit_num))
        //     .execute(conn);
        let raw_sql = format!(
            "UPDATE articles SET visitor_num = visitor_num + {} WHERE id = '{}'",
            self.visit_num,
            self.article_id.to_hyphenated().to_string()
        );
        let res = diesel::sql_query(raw_sql).execute(conn);
        match res {
            Ok(num) => Ok(num),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn update_all(items: Vec<Self>, conn: &PgConnection) -> Result<(), String> {
        let result = conn.transaction(|| {
            for i in items.into_iter() {
                let res = i.update_visit_num(conn);
                if let Err(e) = res {
                    error!("update_visit_num failed: {:?}", e);
                    return Err(diesel::result::Error::RollbackTransaction);
                }
            }

            Ok(())
        });

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("update visit num failed!".to_string()),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ArticleVisitNum {
    pub visitor_num: i64,
}

impl ArticleVisitNum {
    pub fn total_visit_num(conn: &PgConnection) -> Result<i64, String> {
        use crate::schema::articles::dsl::*;
        use bigdecimal::*;
        use diesel::dsl::sum;

        let res = articles
            .select(sum(visitor_num))
            .first::<Option<BigDecimal>>(conn);
        match res {
            Ok(n) => match n {
                Some(v) => Ok(v.to_i64().unwrap()),
                None => Ok(0),
            },
            Err(e) => Err(format!("sum visitor_num failed: {:?}", e)),
        }
    }
}
