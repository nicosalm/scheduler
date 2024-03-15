-- Your SQL goes here

ALTER TABLE users
ADD COLUMN task_count BIGINT,
ADD COLUMN completed_task_count BIGINT,
ADD COLUMN incomplete_task_count BIGINT,
ADD COLUMN overdue_task_count BIGINT;
