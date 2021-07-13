#[macro_use]
extern crate rocket;
extern crate tokio;


use crate::core::rocket_master::rocket;

mod core;
mod tests;

mod controllers;
mod contracts;
mod model;

mod config_controller;
pub mod database;

#[launch]
fn init_main() -> _ {
    rocket()
}