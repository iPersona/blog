CREATE TABLE comment_notify (
    id serial PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users (id),
    comment_id uuid NOT NULL,
    is_read bool NOT NULL DEFAULT FALSE
);

CREATE OR REPLACE FUNCTION trf_keep_user_comment_notify_steady ()
    RETURNS TRIGGER
    AS $BODY$
BEGIN
    IF (
        SELECT
            count(user_id)
        FROM
            comment_notify
        GROUP BY
            user_id
        HAVING
            count(id) > 5) > 0 THEN
        WITH exceed_user AS (
            SELECT
                user_id
            FROM
                comment_notify
            GROUP BY
                user_id
            HAVING
                count(id) > 5
),
exceed_row AS (
    SELECT
        id
    FROM
        comment_notify a
    RIGHT JOIN exceed_user b ON a.user_id = b.user_id
ORDER BY
    a.id
LIMIT 1)
DELETE FROM comment_notify
WHERE id IN (
        SELECT
            id
        FROM
            exceed_row);
END IF;
    RETURN NULL;
    -- select * from comment_notify where id in (select id from exceed_row)
    -- select * from exceed_row;
END;
$BODY$
LANGUAGE plpgsql;

CREATE TRIGGER tr_keep_user_comment_notify_steady
    AFTER INSERT ON comment_notify
    FOR EACH ROW
    EXECUTE PROCEDURE trf_keep_user_comment_notify_steady ();

