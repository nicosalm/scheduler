
// src/main.rs

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/", FileServer::from("/public"))
        .mount("/users", routes![
            routes::get_all_users,
            routes::create_user,
            routes::get_user,
            routes::update_user,
            routes::delete_user,
        ])
        .mount("/tasks", routes![
            routes::create_task,
            routes::get_task,
            routes::get_all_tasks,
            routes::update_task,
            routes::delete_task,
        ])
}
