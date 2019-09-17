use super::super::tags;
use super::super::tags::dsl::tags as all_tags;
use super::Relations;

use crate::AppState;
use diesel;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text, Uuid as sql_uuid};
use std::cell::Ref;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Tags {
    pub id: Uuid,
    pub tag: String,
}

impl Tags {
    pub fn view_list_tag(state: &AppState) -> Result<Vec<Tags>, String> {
        let res = all_tags.load::<Tags>(&state.db.connection());
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    // pub fn get_all(state: &AppState) -> Result<Vec<String>, String> {
    //     let tags = Self::view_list_tag(&state.db.connection());
    //     match tags {
    //         Ok(t) => Ok(t.iter().map(|v| v.tag.clone()).collect()),
    //         Err(e) => Err(e),
    //     }
    // }

    pub fn delete_tag(state: &AppState, id: Uuid) -> Result<usize, String> {
        let conn = &state.db.into_inner().get().unwrap();
        Relations::delete_all(conn, id, "tag");
        let res = diesel::delete(all_tags.filter(tags::id.eq(id))).execute(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn edit_tag(&self, state: &AppState) -> Result<usize, String> {
        let conn = &state.db.into_inner().get().unwrap();
        let res = diesel::update(all_tags.filter(tags::id.eq(&self.id)))
            .set(tags::tag.eq(&self.tag))
            .execute(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, QueryableByName)]
#[table_name = "article_tag_relation"]
pub struct TagCount {
    #[sql_type = "sql_uuid"]
    id: Uuid,
    #[sql_type = "Text"]
    tag: String,
    #[sql_type = "BigInt"]
    count: i64,
}

impl TagCount {
    pub fn view_tag_count(conn: &PgConnection) -> Result<Vec<Self>, String> {
        let res = diesel::sql_query("select b.id, b.tag, count(*) from article_tag_relation a join tags b on a.tag_id=b.id group by b.id, b.tag").load::<Self>(conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }

    pub fn view_all_tag_count(
        state: &AppState,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TagCount>, String> {
        let conn = state.db.into_inner().get().unwrap();
        let raw_sql = format!("select a.id, a.tag, (case when b.count is null then 0 else b.count end) as count from tags a left join \
                (select tag_id, count(*) from article_tag_relation group by tag_id) b on a.id = b.tag_id order by a.id limit {} offset {};", limit, offset);
        let res = diesel::sql_query(raw_sql).load::<Self>(&conn);
        match res {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

#[derive(Insertable, Debug, Clone, Deserialize, Serialize)]
#[table_name = "tags"]
pub struct NewTag {
    tag: String,
}

impl NewTag {
    pub fn new(tag: &str) -> Self {
        NewTag {
            tag: tag.to_owned(),
        }
    }

    pub fn insert(&self, state: &AppState) -> bool {
        let conn = state.db.into_inner().get().unwrap();
        diesel::insert_into(tags::table)
            .values(self)
            .execute(&conn)
            .is_ok()
    }

    pub fn insert_with_result(&self, conn: &PgConnection) -> Tags {
        diesel::insert_into(tags::table)
            .values(self)
            .get_result(conn)
            .unwrap()
    }

    pub fn insert_all(raw_tag: Vec<NewTag>, conn: &PgConnection) -> Vec<Uuid> {
        let new_tags: Vec<Tags> = diesel::insert_into(tags::table)
            .values(&raw_tag)
            .get_results(conn)
            .unwrap();
        new_tags.iter().map(|tag| tag.get_id()).collect()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteTag {
    pub id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewTag {
    pub limit: i64,
    pub offset: i64,
}

impl ViewTag {
    pub fn new(query: Ref<HashMap<String, String>>) -> Option<ViewTag> {
        let limit = query
            .get("limit")
            .map_or(-1, |limit| limit.parse::<i64>().unwrap_or_else(|_| -1));
        let offset = query
            .get("offset")
            .map_or(-1, |offset| offset.parse::<i64>().unwrap_or_else(|_| -1));
        if limit == -1 || offset == -1 {
            return None;
        }
        Some(ViewTag { limit, offset })
    }
}
