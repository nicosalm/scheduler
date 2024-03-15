-- This file should undo anything in `up.sql`

ALTER TABLE users
DROP COLUMN task_count,
DROP COLUMN completed_task_count,
DROP COLUMN incomplete_task_count,
DROP COLUMN overdue_task_count;
