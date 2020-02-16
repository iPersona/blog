-- Mentioned user list
ALTER TABLE comments
ADD COLUMN mentioned_users uuid [];

-- Subscription option
ALTER TABLE users
ADD COLUMN subscribe bool NOT NULL DEFAULT false;
