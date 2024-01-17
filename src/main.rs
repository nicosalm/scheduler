
// main.rs -- main entry point

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod db;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index])
}

