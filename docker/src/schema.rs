table! {
    article_tag_relation (id) {
        id -> Uuid,
        tag_id -> Uuid,
        article_id -> Uuid,
    }
}

table! {
    articles (id) {
        id -> Uuid,
        title -> Varchar,
        raw_content -> Text,
        content -> Text,
        published -> Bool,
        create_time -> Timestamp,
        modify_time -> Timestamp,
        visitor_num -> Int8,
    }
}

table! {
    comments (id) {
        id -> Uuid,
        comment -> Text,
        article_id -> Uuid,
        user_id -> Uuid,
        create_time -> Timestamp,
    }
}

table! {
    daily_statistic (id) {
        id -> Uuid,
        today -> Timestamp,
        visit_num -> Int8,
    }
}

table! {
    tags (id) {
        id -> Uuid,
        tag -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        account -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        groups -> Int2,
        nickname -> Varchar,
        say -> Nullable<Varchar>,
        email -> Varchar,
        disabled -> Int2,
        create_time -> Timestamp,
        github -> Nullable<Varchar>,
    }
}

joinable!(article_tag_relation -> articles (article_id));
joinable!(article_tag_relation -> tags (tag_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));

allow_tables_to_appear_in_same_query!(
    article_tag_relation,
    articles,
    comments,
    daily_statistic,
    tags,
    users,
);