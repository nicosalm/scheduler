
// src/db.rs -- database connection management with r2d2 pooling

#![allow(unused_imports)]
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use crate::schema::{self, users, tasks};
use crate::models::{User, NewUser, UpdateUser, Task, NewTask, UpdateTask};
use rocket::{Rocket, Build, fairing::AdHoc};

type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Pool", |rocket: Rocket<Build>| async {
        let pool = init_pool();
        rocket.manage(pool)
    })
}

// User CRUD operations, adjusted for pooled connections

pub fn create_user(conn: &DbConn, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(conn: &DbConn, id: i32) -> QueryResult<User> {
    users::table.find(id).first(conn)
}

pub fn get_all_users(conn: &DbConn) -> QueryResult<Vec<User>> {
    users::table.load::<User>(conn)
}

pub fn update_user(conn: &DbConn, id: i32, user_data: UpdateUser) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(&user_data)
        .get_result(conn)
}

pub fn delete_user(conn: &DbConn, id: i32) -> QueryResult<usize> {
    diesel::delete(users::table.filter(users::id.eq(id))).execute(conn)
}

// Task CRUD operations, adjusted for pooled connections

pub fn create_task(conn: &DbConn, new_task: NewTask) -> QueryResult<Task> {
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
}

pub fn get_task(conn: &DbConn, task_id: i32) -> QueryResult<Task> {
    tasks::table.find(task_id).first(conn)
}

pub fn get_all_tasks(conn: &DbConn) -> QueryResult<Vec<Task>> {
    tasks::table.load::<Task>(conn)
}

pub fn update_task(conn: &DbConn, task_id: i32, task_data: UpdateTask) -> QueryResult<Task> {
    diesel::update(tasks::table.find(task_id))
        .set(&task_data)
        .get_result(conn)
}

pub fn delete_task(conn: &DbConn, task_id: i32) -> QueryResult<usize> {
    diesel::delete(tasks::table.find(task_id)).execute(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::r2d2::{ConnectionManager, Pool};
    use dotenv::dotenv;
    use diesel::pg::PgConnection;

    fn setup_db() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok(); // load .env
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for tests");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager).expect("Failed to create pool.")
    }

    // utility function to clear data
    fn clear_data(conn: &DbConn) {
        diesel::delete(users::table).execute(conn).unwrap();
        diesel::delete(tasks::table).execute(conn).unwrap();
    }

    #[test]
    fn test_create_user() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");
        clear_data(&conn);

        let new_user = NewUser {
            username: "testuser".into(),
            email: "test@example.com".into(),
            password: "secret".into(),
            is_super: false,
        };

        let user_result = create_user(&conn, new_user).expect("Failed to create user");
        assert_eq!(user_result.username, "testuser");
        assert_eq!(user_result.email, "test@example.com");
    }

    #[test]
    fn test_get_user() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");

        // Assuming 'test_create_user' creates a user, and the ID is known and is 1
        let user = get_user(&conn, 1).expect("Failed to retrieve user");
        assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_update_user() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");
        clear_data(&conn);

        // Assume we have a user with id 1 here
        let updated_data = UpdateUser {
            username: Some("updateduser".into()),
            email: Some("update@example.com".into()),
            password: None,
            is_super: Some(true),
            task_count_current: Some(0),
            task_count_complete: Some(0),
            task_count_incomplete: Some(0),
            task_count_overdue: Some(0),
        };

        let user = update_user(&conn, 1, updated_data).expect("Failed to update user");
        assert_eq!(user.username, "updateduser");
    }

    #[test]
    fn test_delete_user() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");

        let result = delete_user(&conn, 1).expect("Failed to delete user");
        assert_eq!(result, 1); // One record should be deleted
    }

    #[test]
    fn test_create_task() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");

        let new_task = NewTask {
            title: "Finish the report".into(),
            description: Some("The report needs to be finished by tomorrow".into()),
            due: None,
            status: false,
            user_id: Some(1),
        };

        let task = create_task(&conn, new_task).expect("Failed to create task");
        assert_eq!(task.title, "Finish the report");
    }

    // Additional tests for get, update, and delete tasks would follow a similar pattern
}
