table! {
    article_with_tag (id) {
        id -> Uuid,
        title -> Varchar,
        raw_content -> Text,
        content -> Text,
        published -> Bool,
        tags_id -> Array<Nullable<Uuid>>,
        tags -> Array<Nullable<Text>>,
        create_time -> Timestamp,
        modify_time -> Timestamp,
    }
}

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
    comment_notify (id) {
        id -> Int4,
        user_id -> Uuid,
        comment_id -> Uuid,
        is_read -> Bool,
    }
}

table! {
    comments (id) {
        id -> Uuid,
        comment -> Text,
        article_id -> Uuid,
        from_user -> Uuid,
        create_time -> Timestamp,
        mentioned_users -> Nullable<Array<Uuid>>,
        to_user -> Nullable<Uuid>,
        parent_comment -> Nullable<Uuid>,
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
        is_active -> Bool,
        subscribe -> Bool,
    }
}

joinable!(article_tag_relation -> articles (article_id));
joinable!(article_tag_relation -> tags (tag_id));
joinable!(comment_notify -> users (user_id));
joinable!(comments -> articles (article_id));

allow_tables_to_appear_in_same_query!(
    article_tag_relation,
    articles,
    comment_notify,
    comments,
    daily_statistic,
    tags,
    users,
);
