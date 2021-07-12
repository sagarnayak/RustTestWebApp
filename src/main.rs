#[macro_use]
extern crate rocket;
extern crate tokio;

use controllers::error_handlers::not_found;
use controllers::routes;
use database::database_master;

mod core;
mod tests;

mod controllers;
mod model;

mod config_controller;
pub mod database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes::get_routes())
        .manage(database_master::get_db_pools())
}
