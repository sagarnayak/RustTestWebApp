use rocket::Route;

use crate::controllers::database_connected::get_all_user;
use crate::controllers::generic_apis::blocking_task;
use crate::controllers::generic_apis::get_accepted;
use crate::controllers::generic_apis::get_status_code;
use crate::controllers::generic_apis::get_status_code_dfvdfb;
use crate::controllers::generic_apis::index;
use crate::controllers::generic_apis::may_get;
use crate::controllers::generic_apis::me;
use crate::controllers::generic_apis::test_number_exception;
use crate::controllers::generic_apis::world;
use crate::controllers::json_response::get_json_response;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        world,
        may_get,
        blocking_task,
        me,
        get_json_response,
        get_all_user,
        get_accepted,
        get_status_code,
        test_number_exception,
        get_status_code_dfvdfb,
    ]
}