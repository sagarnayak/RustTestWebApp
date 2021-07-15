#[macro_use]
extern crate rocket;
extern crate tokio;


use schedules::schedule_master::schedule;

use crate::core::rocket_master::rocket;

mod core;
mod tests;

mod controllers;
mod contracts;
mod enums;
mod model;
mod schedules;
mod config_controller;
pub mod database;

#[launch]
fn init_main() -> _ {
    schedule();
    rocket()
}