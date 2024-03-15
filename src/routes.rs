
// src/routes.rs -- web endpoints

#![allow(dead_code)]

use rocket::{State, get, post, put, delete, http::Status, serde::json::Json};
use crate::db::{self, Pool};
use crate::models::{User, NewUser, UpdateUser, Task, NewTask, UpdateTask};

#[get("/users")]
pub fn get_all_users(pool: &State<Pool>) -> Result<Json<Vec<User>>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::get_all_users(&conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/users", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>, pool: &State<Pool>) -> Result<Json<User>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::create_user(&conn, user.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/users/<id>")]
pub fn get_user(id: i32, pool: &State<Pool>) -> Result<Json<User>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::get_user(&conn, id)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[put("/users/<id>", format = "json", data = "<user_data>")]
pub fn update_user(id: i32, user_data: Json<UpdateUser>, pool: &State<Pool>) -> Result<Json<User>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::update_user(&conn, id, user_data.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/users/<id>")]
pub fn delete_user(id: i32, pool: &State<Pool>) -> Result<Json<usize>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::delete_user(&conn, id)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/tasks", format = "json", data = "<task>")]
pub fn create_task(task: Json<NewTask>, pool: &State<Pool>) -> Result<Json<Task>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::create_task(&conn, task.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/tasks/<id>")]
pub fn get_task(id: i32, pool: &State<Pool>) -> Result<Json<Task>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::get_task(&conn, id)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/tasks")]
pub fn get_all_tasks(pool: &State<Pool>) -> Result<Json<Vec<Task>>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::get_all_tasks(&conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[put("/tasks/<id>", format = "json", data = "<task_data>")]
pub fn update_task(id: i32, task_data: Json<UpdateTask>, pool: &State<Pool>) -> Result<Json<Task>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::update_task(&conn, id, task_data.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/tasks/<id>")]
pub fn delete_task(id: i32, pool: &State<Pool>) -> Result<Json<usize>, Status> {
    let conn = pool.get().expect("Failed to get a database connection from the pool");
    db::delete_task(&conn, id)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
