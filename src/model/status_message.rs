use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessage {
    pub code: u16,
    pub message: String,
}

impl StatusMessage {
    pub fn bad_req(message: String) -> StatusMessage {
        StatusMessage {
            code: Status::BadRequest.code,
            message,
        }
    }
    pub fn bad_req_default() -> StatusMessage {
        StatusMessage {
            code: Status::BadRequest.code,
            message: "Getting some error ...".to_string(),
        }
    }
}