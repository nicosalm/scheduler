// models.rs -- structs for querying data

use chrono::NaiveDate;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use diesel::{Insertable, Queryable, AsChangeset, Associations};
use crate::schema::{users, tasks};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};
use std::io::Write;
use std::fmt;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub task_count: i64,
    pub completed_task_count: i64,
    pub incomplete_task_count: i64,
    pub overdue_task_count: i64,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub task_count: Option<i64>,
    pub completed_task_count: Option<i64>,
    pub incomplete_task_count: Option<i64>,
    pub overdue_task_count: Option<i64>,
}

#[derive(Queryable, Serialize, Deserialize, Associations)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: bool,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: bool,
    pub user_id: Option<i32>,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<bool>,
    pub user_id: Option<i32>,
}
