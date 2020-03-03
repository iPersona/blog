-- drop table
DROP TABLE comment_notify;

-- drop trigger
DROP TRIGGER IF EXISTS tr_keep_user_comment_notify_steady ON comment_notify;

-- drop trigger function
DROP FUNCTION trf_keep_user_comment_notify_steady ();

