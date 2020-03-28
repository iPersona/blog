CREATE OR REPLACE FUNCTION get_comment_notifications (_user_id uuid) RETURNS TABLE (j json) LANGUAGE plpgsql AS $$
BEGIN
    RETURN query WITH cmt AS (
        SELECT
            b.id AS comment_id,
            b.article_id,
            b.create_time,
            a.user_id,
            a.is_read
        FROM
            comment_notify a
        LEFT JOIN comments b ON a.comment_id = b.id
    WHERE
        a.user_id = _user_id
    ORDER BY
        a.id DESC
),
atc AS (
    SELECT
        a.comment_id,
        a.user_id,
        a.article_id,
        a.create_time,
        b.title AS article_title,
        a.is_read
    FROM
        cmt a
    LEFT JOIN articles b ON a.article_id = b.id
),
usr AS (
    SELECT
        a.comment_id,
        a.user_id,
        b.nickname,
        a.article_id,
        a.article_title,
        a.create_time,
        a.is_read
    FROM
        atc a
        LEFT JOIN users b ON a.user_id = b.id
),
data AS (
    SELECT
        *
    FROM
        usr
)
SELECT
    json_strip_nulls(row_to_json(data))
FROM
    data;
END;
$$;