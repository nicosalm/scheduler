
// models.rs -- structs for querying data

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable, AsChangeset, Associations};
use crate::schema::{users, tasks};
use diesel::backend::Backend;
use diesel::serialize::{self, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};
use diesel::pg::Pg;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: UserRole,
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
    pub role: Option<UserRole>,
}

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq)]
#[sql_type = "Text"]
pub enum UserRole {
    Admin,
    Member,
}

impl<DB> ToSql<Text, DB> for UserRole
where
    DB: Backend,
    for<'a> &'a str: ToSql<Text, DB>,
{
    fn to_sql<W: std::io::Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        match *self {
            UserRole::Admin => <&str as ToSql<Text, DB>>::to_sql(&"Admin", out),
            UserRole::Member => <&str as ToSql<Text, DB>>::to_sql(&"Member", out),
        }
    }
}

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Admin" => Ok(UserRole::Admin),
            b"Member" => Ok(UserRole::Member),
            _ => Err("Unrecognized UserRole".into()),
        }
    }
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
