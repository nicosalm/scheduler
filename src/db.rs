
// src/db.rs -- databse connection

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::schema::users::dsl::*;

// Establish a connection to the database
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect("Error connecting to database")
}

// User CRUD operations

use crate::models::{User, NewUser, UpdateUser};

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

// Task CRUD operations

use crate::models::{Task, NewTask, UpdateTask};

pub fn create_task(conn: &PgConnection, new_task: NewTask) -> QueryResult<Task> {
    use crate::schema::tasks::dsl::*;
    diesel::insert_into(tasks)
        .values(&new_task)
        .get_result(conn)
}

pub fn get_task(conn: &PgConnection, task_id: i32) -> QueryResult<Task> {
    use crate::schema::tasks::dsl::*;
    tasks.find(task_id).first(conn)
}

pub fn get_all_tasks(conn: &PgConnection) -> QueryResult<Vec<Task>> {
    use crate::schema::tasks::dsl::*;
    tasks.load::<Task>(conn)
}

pub fn update_task(conn: &PgConnection, task_id: i32, task_data: UpdateTask) -> QueryResult<Task> {
    use crate::schema::tasks::dsl::*;
    diesel::update(tasks.find(task_id))
        .set(&task_data)
        .get_result(conn)
}

pub fn delete_task(conn: &PgConnection, task_id: i32) -> QueryResult<usize> {
    use crate::schema::tasks::dsl::*;
    diesel::delete(tasks.find(task_id)).execute(conn)
}



// --- TESTS ---

#[cfg(test)]
mod db_tests {
    use super::*;
    # [allow(unused_imports)]
    use crate::models::{NewUser, NewTask};
    use diesel::connection::TransactionManager;
    use diesel::Connection;

    #[test] 
    fn test_create_task() {
        let conn = establish_connection();
        let transaction = conn.transaction_manager();
        transaction.begin_transaction(&conn).unwrap();

        let new_task = NewTask {
            title: "test task".to_string(),
            description: Some("test description".to_string()),
            due_date: None,
            status: false,
            user_id: None,
        };
        let result = create_task(&conn, new_task);

        assert!(result.is_ok());
        let task = result.unwrap(); // Unwrap once and use the value
        assert_eq!(task.title, "test task");
        assert_eq!(task.description, Some("test description".to_string()));
        assert_eq!(task.due_date, None);
        assert_eq!(task.status, false);
        assert_eq!(task.user_id, None);
        
        // Rollback the transaction so we don't actually modify the database
        transaction.rollback_transaction(&conn).unwrap();

    }
}

