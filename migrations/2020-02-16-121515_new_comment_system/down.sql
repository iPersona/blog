-- Rename comment sender
ALTER TABLE comments
    RENAME COLUMN from_user TO user_id;

-- Add comment receiver, default to admin
ALTER TABLE comments
    DROP COLUMN to_user;

-- Add parent column
ALTER TABLE comments
    DROP COLUMN parent_comment;

-- ALTER TABLE comments
--     DROP COLUMN sub_comments_num;
