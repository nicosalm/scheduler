
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    is_super BOOLEAN NOT NULL,
    task_count_current BIGINT,
    task_count_complete BIGINT,
    task_count_incomplete BIGINT,
    task_count_overdue BIGINT
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    due DATE,
    status BOOLEAN NOT NULL,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX idx_tasks_user_id ON tasks(user_id);
