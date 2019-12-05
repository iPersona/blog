CREATE TABLE daily_statistic
(
    id uuid primary key default gen_random_uuid(),
    today timestamp NOT NULL default current_timestamp,
    visit_num bigint NOT NULL DEFAULT 0
);