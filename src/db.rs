// src/db.rs -- database connection management with r2d2 pooling

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use crate::schema::users::dsl::*;
use crate::models::{User, NewUser, UpdateUser, Task, NewTask, UpdateTask};
use rocket::{Rocket, Build, fairing::AdHoc};

// Type aliases for convenience
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// Initialize the connection pool
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
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(conn: &DbConn, user_id: i32) -> QueryResult<User> {
    users.find(user_id).first(conn)
}

pub fn get_all_users(conn: &DbConn) -> QueryResult<Vec<User>> {
    users.load::<User>(conn)
}

pub fn update_user(conn: &DbConn, user_id: i32, user_data: UpdateUser) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set(&user_data)
        .get_result(conn)
}

pub fn delete_user(conn: &DbConn, user_id: i32) -> QueryResult<usize> {
    diesel::delete(users.find(user_id)).execute(conn)
}

// Task CRUD operations, adjusted for pooled connections

pub fn create_task(conn: &DbConn, new_task: NewTask) -> QueryResult<Task> {
    use crate::schema::tasks::dsl::*;
    diesel::insert_into(tasks)
        .values(&new_task)
        .get_result(conn)
}

pub fn get_task(conn: &DbConn, task_id: i32) -> QueryResult<Task> {
    use crate::schema::tasks::dsl::*;
    tasks.find(task_id).first(conn)
}

pub fn get_all_tasks(conn: &DbConn) -> QueryResult<Vec<Task>> {
    use crate::schema::tasks::dsl::*;
    tasks.load::<Task>(conn)
}

pub fn update_task(conn: &DbConn, task_id: i32, task_data: UpdateTask) -> QueryResult<Task> {
    use crate::schema::tasks::dsl::*;
    diesel::update(tasks.find(task_id))
        .set(&task_data)
        .get_result(conn)
}

pub fn delete_task(conn: &DbConn, task_id: i32) -> QueryResult<usize> {
    use crate::schema::tasks::dsl::*;
    diesel::delete(tasks.find(task_id)).execute(conn)
}

// Tests

mod tests {
    use super::*; // Import everything from the outer module
    use diesel::r2d2::{ConnectionManager, Pool};
    use dotenv::dotenv;
    use diesel::pg::PgConnection;

    fn setup_db() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok(); // Load .env file if available
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for tests");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager).expect("Failed to create pool.")
    }

    #[test]
    fn test_create_user() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");
    
        // Your test logic here... 
    }

    #[test]
    fn test_create_task() {
        let pool = setup_db();
        let conn = pool.get().expect("Failed to get a database connection from the pool");
    
        // Test creating a new task similarly
    }
}
