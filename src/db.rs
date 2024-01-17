
// src/db.rs -- databse connection

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::users::dsl::*;

use crate::models::{User, NewUser, UpdateUser};

// Establish a connection to the database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect("Error connecting to database")
}

// User CRUD operations

pub fn create_user(conn: &PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(conn: &PgConnection, user_id: i32) -> QueryResult<User> {
    users.find(user_id).first(conn)
}

pub fn get_all_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn update_user(conn: &PgConnection, user_id: i32, user_data: UpdateUser) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set(&user_data)
        .get_result(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: i32) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}

