use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use rocket::http::Status;
use rocket::response::content;
use rocket::response::status;

use crate::model::user::User;

#[get("/")]
pub fn index() -> &'static str {
    "Hello!"
}

#[get("/world")]
pub fn world() -> &'static str {
    "hello, world!"
}

#[get("/mayGet")]
pub fn may_get() -> Result<String, String> {
    let result_to_return = rand::thread_rng().gen_bool(0.5);
    if result_to_return {
        Ok("result ...".to_string())
    } else {
        Err("error...".to_string())
    }
}

#[get("/blockingTask")]
pub async fn blocking_task() -> Result<String, String> {
    sleep(Duration::from_secs(rand::thread_rng().gen_range(2..6)));
    Ok("Done .".to_string())
}

#[get("/me/<name>")]
pub fn me(name: &str) -> String {
    let result_to_return = format!("hi, {}", name);
    result_to_return
}

#[get("/testNumberException/<number>")]
pub fn test_number_exception(number: Result<u8, &str>) -> String {
    match number {
        Ok(number_positive) => "success".to_string(),
        Err(error) => "error".to_string(),
    }
}

#[get("/getAccepted")]
pub fn get_accepted() -> status::Accepted<String> {
    status::Accepted(Some(format!("id:")))
}

#[get("/getStatusCode")]
pub fn get_status_code() -> status::Custom<content::Json<&'static str>> {
    status::Custom(Status::ImATeapot, content::Json("{ \"hi\": \"world\" }"))
}

#[get("/getStatusCodeTwo")]
pub fn get_status_code_dfvdfb() -> status::Custom<content::Json<User>> {
    status::Custom(Status::BadRequest, content::Json(User { id: 34, name: "sagar".to_string(), email: "sgar@gmail.com".to_string() }))
}

#[get("/getBlankArray")]
pub fn get_blank_array() -> status::Custom<content::Json<Vec<User>>> {
    status::Custom(Status::Ok, content::Json(vec![]))
}