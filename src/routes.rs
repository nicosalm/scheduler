
// src/routes.rs -- web endpoints

#![allow(dead_code)]

use rocket::{get, post, put, delete};
use rocket::http::Status;
use rocket::serde::json::Json;
use diesel::prelude::*;

use crate::db;
use crate::models::{User, NewUser, UpdateUser};

#[get("/")]
pub fn index() -> &'static str {
    // Attempt to connect to the database
    let _connection = db::establish_connection();

    // Here you can make queries to the database using `_connection`
    // For now, we'll just return a simple message
    "Connected to the PostgreSQL database!"
}

// User CRUD operations

#[get("/users")]
pub fn get_users() -> String { 
    use crate::schema::users::dsl::*; // import the `users` table
    
    let connection = db::establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    let mut response = String::new();
    for user in results {
        response.push_str(&format!("{} : {}\n", user.id, user.username));
    }

    response
}

#[post("/users", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Json<User> {
    let connection = db::establish_connection();
    Json(db::create_user(&connection, user.into_inner()).unwrap())
}

#[get("/users")]
pub fn get_all_users() -> Json<Vec<User>> {
    let connection = db::establish_connection();
    Json(db::get_all_users(&connection).unwrap())
}

#[get("/users/<id>")]
pub fn get_user(id: i32) -> Json<User> {
    let connection = db::establish_connection();
    Json(db::get_user(&connection, id).unwrap())
}

#[put("/users/<id>", format = "json", data = "<user_data>")]
pub fn update_user(id: i32, user_data: Json<UpdateUser>) -> Result<Json<User>, Status> {
    let connection = db::establish_connection();
    db::update_user(&connection, id, user_data.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)    
}

#[delete("/users/<id>")]
pub fn delete_user(id: i32) -> Json<usize> {
    let connection = db::establish_connection();
    Json(db::delete_user(&connection, id).unwrap())
}

// Task CRUD operations

use crate::models::{Task, NewTask, UpdateTask};

#[post("/tasks", format = "json", data = "<task>")]
pub fn create_task(task: Json<NewTask>) -> Json<Task> {
    let connection = db::establish_connection();
    Json(db::create_task(&connection, task.into_inner()).unwrap())
}

#[get("/tasks")]
pub fn get_all_tasks() -> Json<Vec<Task>> {
    let connection = db::establish_connection();
    Json(db::get_all_tasks(&connection).unwrap())
}

#[get("/tasks/<id>")]
pub fn get_task(id: i32) -> Json<Task> {
    let connection = db::establish_connection();
    Json(db::get_task(&connection, id).unwrap())
}

#[put("/tasks/<id>", format = "json", data = "<task_data>")]
pub fn update_task(id: i32, task_data: Json<UpdateTask>) -> Result<Json<Task>, Status> {
    let connection = db::establish_connection();
    db::update_task(&connection, id, task_data.into_inner())
        .map(Json)
        .map_err(|_| Status::InternalServerError)    
}

#[delete("/tasks/<id>")]
pub fn delete_task(id: i32) -> Json<usize> {
    let connection = db::establish_connection();
    Json(db::delete_task(&connection, id).unwrap())
}
