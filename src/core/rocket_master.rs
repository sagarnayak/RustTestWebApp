use rocket::{Build, Rocket};

use crate::controllers::error_handlers::not_found;
use crate::controllers::routes;
use crate::database::database_master;

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes::get_routes())
        .manage(database_master::get_db_pools())
}