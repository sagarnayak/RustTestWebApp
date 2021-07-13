extern crate bcrypt;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::Json;

use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[post("/authenticate", data = "<user>")]
pub fn authenticate_user(user: Json<User>, _key: ApiKey<'_>)
                         -> status::Custom<Result<String, StatusMessage>> {
    status::Custom(
        Status::Ok,
        Ok(format!("Hi {}, you are authenticated.", user.name)),
    )
}

pub struct ApiKey<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = StatusMessage;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, StatusMessage> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::Unauthorized, StatusMessage { code: 401, message: "You are not authorised".to_string() })),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, StatusMessage { code: 401, message: "You are not authorised".to_string() })),
        }
    }
}