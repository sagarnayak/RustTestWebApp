use rocket::http::Status;
use rocket::Request;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::model::status_message::StatusMessage;

#[catch(default)]
pub fn not_found(status: Status, req: &Request) -> status::Custom<Json<StatusMessage>> {
    println!("data :: {:?}", req.client_ip());
    println!("data :: {:?}", req.headers());
    status::Custom(
        status,
        Json(
            StatusMessage {
                code: status.code,
                message: format!("we are unable to find the path :: {}", req.uri()),
            }
        ),
    )
}