CREATE OR REPLACE FUNCTION mark_notification_read(ntyid bigint) RETURNS void LANGUAGE plpgsql AS $$
BEGIN
IF (SELECT COUNT(id) FROM comment_notify WHERE id = ntyid AND is_read = false) > 0 THEN
    UPDATE comment_notify
    SET is_read = true
    WHERE id = ntyid;
END IF;
END;
$$;

-- remove old function

DROP FUNCTION IF EXISTS comment_data (uuid, uuid, bigint);

-- get location data of comment data

CREATE OR REPLACE FUNCTION comment_data (_article_id uuid, comment_id uuid, comment_notify_id bigint, page_size bigint) RETURNS TABLE (j json) LANGUAGE plpgsql AS $$
BEGIN
    -- mark target comment readed
    PERFORM mark_notification_read(comment_notify_id);
    -- get target comment data
    IF (
        SELECT
            count(id)
        FROM
            comments
        WHERE
            id = comment_id AND parent_comment IS NULL) > 0 THEN
        -- target comment without sub comments
        RETURN query WITH data AS (
            SELECT
                parent_comment_data (_article_id,
                    comment_id,
                    page_size) AS parent
)
        SELECT
            row_to_json(data)
        FROM
            data;
    ELSE
        -- target comment with sub comments
        RETURN query WITH data AS (
            SELECT
                sub_comment_parent_data (_article_id,
                    comment_id,
                    page_size) AS parent,
                sub_comment_child_data (comment_id, page_size) AS child
)
        SELECT
            json_strip_nulls (row_to_json(data))
        FROM
            data;
    END IF;
END;
$$;