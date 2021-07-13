use rocket::Route;

use crate::controllers::authentication_with_guard::authenticate_user;
use crate::controllers::database_connected::get_all_user;
use crate::controllers::generic_apis::blocking_task;
use crate::controllers::generic_apis::get_accepted;
use crate::controllers::generic_apis::get_blank_array;
use crate::controllers::generic_apis::get_status_code;
use crate::controllers::generic_apis::get_status_code_dfvdfb;
use crate::controllers::generic_apis::index;
use crate::controllers::generic_apis::may_get;
use crate::controllers::generic_apis::me;
use crate::controllers::generic_apis::test_number_exception;
use crate::controllers::generic_apis::world;
use crate::controllers::json_response::get_json_response;
use crate::controllers::json_web_token_test::create_jwt;
use crate::controllers::json_web_token_test::verify_jwt;
use crate::controllers::passwords::hash_my_password;
use crate::controllers::passwords::verify_hashed_password;

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
        get_blank_array,
        hash_my_password,
        verify_hashed_password,
        authenticate_user,
        create_jwt,
        verify_jwt,
    ]
}