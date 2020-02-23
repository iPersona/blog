-- Rename comment sender
ALTER TABLE comments
RENAME COLUMN user_id TO from_user;

-- Add comment receiver, default to admin
ALTER TABLE comments
ADD COLUMN to_user uuid REFERENCES users (id);

-- Add parent column
ALTER TABLE comments
ADD COLUMN parent_comment uuid REFERENCES comments (id);

-- Add sub comments number
-- ALTER TABLE comments
-- ADD COLUMN sub_comments_num BIGINT;
