use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::encode;
use jsonwebtoken::errors::ErrorKind;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::config_controller::ConfigData;
use crate::core::strings::FAILED_TO_GET_CONFIG_DATA;
use crate::enums::access_policy::AccessPolicy;
use crate::model::jwt_validation_req::JWTValidationRequest;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
    access_policy: AccessPolicy,
}

#[get("/createJWT")]
pub fn create_jwt() -> String {
    let config = ConfigData::new().expect(FAILED_TO_GET_CONFIG_DATA);
    let jwt_config = config.jwt;

    let key = jwt_config.secret.as_bytes();

    let my_claims =
        Claims {
            sub: "sagar@gmail.com".to_owned(),
            company: "AGPayTech".to_owned(),
            exp: (Utc::now().timestamp() + 30) as usize,
            access_policy: AccessPolicy::Admin,
        };

    let mut header = Header::default();
    header.kid = Some("key identifier".to_string());
    header.alg = Algorithm::HS512;
    let token = match encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(key),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    token
}

#[post("/verifyJWT", data = "<jwt_validation_request>")]
pub fn verify_jwt(jwt_validation_request: Json<JWTValidationRequest>) -> String {
    let config = ConfigData::new().expect(FAILED_TO_GET_CONFIG_DATA);
    let jwt_config = config.jwt;

    let key = jwt_config.secret.as_bytes();

    let token_data = match decode::<Claims>(
        &jwt_validation_request.token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => c,
        Err(err) => {
            println!("the error kind is {}", &err);
            return match *err.kind() {
                ErrorKind::InvalidToken => "Token is invalid".to_owned(),
                ErrorKind::InvalidIssuer => "Issuer is invalid".to_owned(),
                ErrorKind::ExpiredSignature => "expired".to_owned(),
                _ => "Other error".to_owned(),
            };
        }
    };

    println!("the token data is : {:?}", &token_data);

    match token_data.claims.access_policy {
        AccessPolicy::Admin => println!("this is an admin"),
        _ => println!("i dont know you.")
    }

    "done ".to_owned()
}