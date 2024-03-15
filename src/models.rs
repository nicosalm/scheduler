// models.rs -- structs for querying data

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable, AsChangeset, Associations};
use crate::schema::{users, tasks};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};
use std::io::Write;

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum UserRole {
    Admin,
    Member,
}

impl ToSql<Text, Pg> for UserRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match self {
            UserRole::Admin => out.write_all(b"admin")?,
            UserRole::Member => out.write_all(b"member")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(bytes: Option<&<Pg as diesel::backend::Backend>::RawValue>) -> deserialize::Result<Self> {
        match <String as FromSql<Text, Pg>>::from_sql(bytes)?.as_str() {
            "admin" => Ok(UserRole::Admin),
            "member" => Ok(UserRole::Member),
            _ => Err("Unrecognized user role".into()),
        }
    }
}

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
    pub role: UserRole,
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
