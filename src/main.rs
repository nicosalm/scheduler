
// main.rs -- main entry point

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod db;
pub mod models;
pub mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index, routes::create_user])
}

