DROP FUNCTION IF EXISTS comment_data (uuid, uuid, bigint);


DROP FUNCTION IF EXISTS sub_comment_parent_data (uuid, uuid, bigint);


DROP FUNCTION IF EXISTS sub_comment_child_data (uuid, bigint);


DROP FUNCTION IF EXISTS parent_comment_data (uuid, uuid, bigint);


DROP FUNCTION IF EXISTS page_num (bigint, bigint);