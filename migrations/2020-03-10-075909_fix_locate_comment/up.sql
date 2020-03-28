-- get comment data without sub comments

CREATE OR REPLACE FUNCTION parent_comment_data (_article_id uuid, comment_id uuid, page_size bigint) RETURNS TABLE (j json) LANGUAGE plpgsql AS $$
BEGIN
    RETURN query
    -- get first class comment data
    WITH cmt_of_art AS (
    SELECT
        a.id,
        a.comment,
        a.article_id,
        a.from_user as user_id,
        b.nickname,
        a.create_time
    FROM
        comments a
    LEFT JOIN users b ON a.from_user = b.id
WHERE
    a.article_id = _article_id
    AND a.parent_comment IS NULL
ORDER BY
    create_time DESC
),
-- get parent comment data with row number
cmt_with_row_num AS (
    SELECT
        row_number() OVER (ORDER BY create_time DESC) AS rn,
        id
    FROM
        cmt_of_art
),
-- get page number of the target comment
cmt_page_num AS (
    SELECT
        page_num (rn,
            page_size) AS page_num
    FROM
        cmt_with_row_num
    WHERE
        id = comment_id
),
-- get the target comment page data
cmt_page AS (
    SELECT
        *
    FROM
        cmt_of_art offset (
            SELECT
                (page_num - 1) * page_size
            FROM
                cmt_page_num)
        LIMIT page_size
),
-- generate location data
data AS (
    SELECT
        (
            SELECT
                count(id) AS total
            FROM
                cmt_of_art),
            (
                SELECT
                    page_num AS page
                FROM
                    cmt_page_num),
                array_agg(cmt_page) AS comments
            FROM
                cmt_page
)
    SELECT
        json_strip_nulls (row_to_json(data))
FROM
    data;
END;
$$;

-------------------------------
-- get child data of comment with sub comments

CREATE OR REPLACE FUNCTION sub_comment_child_data (comment_id uuid, page_size bigint) RETURNS TABLE (j json) LANGUAGE plpgsql AS $$
BEGIN
    -- get parent comment id
    RETURN query WITH p_id AS (
        SELECT
            parent_comment
        FROM
            comments
        WHERE
            id = comment_id
),
-- get all sub comments
s_cms AS (
    SELECT
        a.id,
        a.comment,
        a.article_id,
        a.from_user as user_id,
        b.nickname,
        a.create_time
    FROM
        comments a
    LEFT JOIN users b ON a.from_user = b.id
WHERE
    a.parent_comment = (
        SELECT
            parent_comment
        FROM
            p_id)
    ORDER BY
        create_time DESC
),
-- get sub comment id with row number
s_cms_with_row_num AS (
    SELECT
        row_number() OVER (ORDER BY create_time DESC) AS rn,
        id
    FROM
        s_cms
),
-- get page number where the target comment in
s_page_num AS (
    SELECT
        page_num (rn,
            page_size) AS page_num
    FROM
        s_cms_with_row_num
    WHERE
        id = comment_id
),
-- get page data of sub comments
s_cms_page AS (
    SELECT
        *
    FROM
        s_cms offset (
            SELECT
                (page_num - 1) * page_size
            FROM
                s_page_num)
        LIMIT page_size
),
-- get sub comment location data
data AS (
    SELECT
        (
            SELECT
                parent_comment AS pid
            FROM
                p_id),
            (
                SELECT
                    count(id) AS total
                FROM
                    s_cms),
                (
                    SELECT
                        page_num AS page
                    FROM
                        s_page_num),
                    array_agg(s_cms_page) AS comments
                FROM
                    s_cms_page
)
        SELECT
            json_strip_nulls (row_to_json(data))
    FROM
        data;
END;
$$;

-- get parent data of comment with sub comments

CREATE OR REPLACE FUNCTION sub_comment_parent_data (_article_id uuid, comment_id uuid, page_size bigint) RETURNS TABLE (j json) LANGUAGE plpgsql AS $$
BEGIN
    -- get parent comment id
    RETURN query WITH p_id AS (
        SELECT
            parent_comment
        FROM
            comments
        WHERE
            id = comment_id
),
-- count sub comment numbers
sub_comments_num AS (
    SELECT DISTINCT
        parent_comment,
        count(*) OVER (PARTITION BY parent_comment) AS sub_comments_num
    FROM
        comments
    WHERE
        article_id = _article_id
        AND parent_comment IS NOT NULL
),
-- get parent comments of the target
p_cms_no_s_cms_num AS (
    SELECT
        a.id,
        a.comment,
        a.article_id,
        a.from_user as user_id,
        b.nickname,
        a.create_time
    FROM
        comments a
    LEFT JOIN users b ON a.from_user = b.id
WHERE
    a.parent_comment IS NULL
ORDER BY
    create_time DESC
),
-- append sub comment number into record
p_cms AS (
    SELECT
        *
    FROM
        p_cms_no_s_cms_num p
    LEFT JOIN sub_comments_num s ON p.id = s.parent_comment
ORDER BY
    p.create_time DESC
),
-- get parent comment data with row number
p_cms_with_row_num AS (
    SELECT
        row_number() OVER (ORDER BY create_time DESC) AS rn,
        id
    FROM
        p_cms
),
-- get page number of the target parent comment
p_page_num AS (
    SELECT
        page_num (rn,
            page_size) AS page_num
    FROM
        p_cms_with_row_num
    WHERE
        id = (
            SELECT
                parent_comment
            FROM
                p_id)
),
-- get data of the target parent comment
p_cms_page AS (
    SELECT
        *
    FROM
        p_cms offset (
            SELECT
                (page_num - 1) * page_size
            FROM
                p_page_num)
        LIMIT page_size
),
-- get location data of target parent comment
data AS (
    SELECT
        (
            SELECT
                count(id) AS total
            FROM
                p_cms),
            (
                SELECT
                    page_num AS page
                FROM
                    p_page_num),
                array_agg(p_cms_page) AS comments
            FROM
                p_cms_page
)
    SELECT
        json_strip_nulls (row_to_json(data))
FROM
    data;
END;
$$;

