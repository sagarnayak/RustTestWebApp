extern crate bcrypt;

use bcrypt::{hash, verify};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::model::password_verification_req::PasswordVerificationRequest;
use crate::model::status_message::StatusMessage;

const B_CRYPT_COST: u32 = 12;

#[get("/hashMyPassword/<password>")]
pub fn hash_my_password(
    password: Result<String, &str>
) -> status::Custom<Result<String, StatusMessage>> {
    let password_to_hash = match password {
        Ok(password_positive) => password_positive,
        Err(error) => return status::Custom(
            Status::BadRequest,
            Err(StatusMessage::bad_req(error.to_string())),
        )
    };

    let hashed = hash(password_to_hash, B_CRYPT_COST);
    let hashed = match hashed {
        Ok(hashed_positive) => hashed_positive,
        Err(error) => return status::Custom(
            Status::BadRequest,
            Err(StatusMessage::bad_req(error.to_string())),
        ),
    };

    status::Custom(Status::Ok, Ok(hashed))
}

#[post("/verifyHashedMyPassword", data = "<password_verification_request>")]
pub fn verify_hashed_password(
    password_verification_request: Json<PasswordVerificationRequest>,
) -> status::Custom<Result<String, StatusMessage>> {
    let original_password = password_verification_request.original_password.clone();

    let password_hash = password_verification_request.hash.clone();

    let valid = verify(original_password, &password_hash);

    return match valid {
        Ok(valid_positive) => match valid_positive {
            true => status::Custom(
                Status::Ok,
                Ok(
                    "Good password".to_string()
                ),
            ),
            false => status::Custom(
                Status::Ok,
                Ok(
                    "Bad password".to_string()
                ),
            ),
        },
        Err(error) => status::Custom(
            Status::BadRequest,
            Err(StatusMessage::bad_req(error.to_string())),
        ),
    };
}