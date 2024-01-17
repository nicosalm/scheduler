
// models.rs -- structs for querying data

use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable, AsChangeset};
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
}
