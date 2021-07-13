use rand::Rng;
use rocket::serde::json::Json;

use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/getUsers")]
pub fn get_json_response() -> Result<Json<User>, Json<StatusMessage>> {
    let random_result_to_send: bool = rand::thread_rng().gen_bool(1.0 / 3.0);

    if random_result_to_send {
        Ok(
            Json(
                User {
                    id: 32,
                    name: "User name".to_string(),
                    email_id: "email".to_string(),
                }
            )
        )
    } else {
        Err(
            Json(
                StatusMessage {
                    code: 400,
                    message: "got an error".to_string(),
                }
            )
        )
    }
}

