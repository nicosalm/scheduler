
// models.rs -- structs for querying data

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable, AsChangeset, Associations};
use crate::schema::{users, tasks};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String, // Ensure the password field is handled securely.
    pub is_super: bool,
    pub task_count_current: Option<i64>,
    pub task_count_complete: Option<i64>,
    pub task_count_incomplete: Option<i64>,
    pub task_count_overdue: Option<i64>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String, // Ensure passwords are hashed before being saved
    pub is_super: bool,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>, // Consider security implications
    pub is_super: Option<bool>,
    pub task_count_current: Option<i64>,
    pub task_count_complete: Option<i64>,
    pub task_count_incomplete: Option<i64>,
    pub task_count_overdue: Option<i64>,
}

#[derive(Queryable, Serialize, Deserialize, Associations)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub due: Option<NaiveDate>,
    pub status: bool,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due: Option<NaiveDate>,
    pub status: bool,
    pub user_id: Option<i32>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due: Option<NaiveDate>,
    pub status: Option<bool>,
    pub user_id: Option<i32>,
}
