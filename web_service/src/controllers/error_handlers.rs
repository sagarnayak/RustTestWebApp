use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;

use crate::model::status_message::StatusMessage;

#[catch(default)]
pub fn not_found(status: Status, req: &Request) -> Json<StatusMessage> {
    Json(
        StatusMessage {
            code: i32::from(status.code),
            message: format!("we are unable to find the path :: {}", req.uri()),
        }
    )
}