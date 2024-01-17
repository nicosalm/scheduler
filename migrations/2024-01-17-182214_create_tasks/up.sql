-- Your SQL goes here

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date DATE,
    status BOOLEAN NOT NULL,
    user_id INTEGER REFERENCES users(id)
);

