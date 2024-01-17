
// main.rs -- main entry point

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod db;
pub mod models;
pub mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    let user_routes = routes![
        routes::create_user,
        routes::get_user,
        routes::get_all_users,
        routes::update_user,
        routes::delete_user,
    ];

    let task_routes = routes![
        routes::create_task,
        routes::get_task,
        routes::get_all_tasks,
        routes::update_task,
        routes::delete_task,
    ];

    rocket::build()
        .mount("/users", user_routes)
        .mount("/tasks", task_routes)
}

