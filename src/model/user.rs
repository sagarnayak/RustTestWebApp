use std::io::Cursor;

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

pub const TABLE_NAME_USER: &str = "users";

impl<'r> Responder<'r, 'static> for User {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json_string = rocket::serde::json::serde_json::to_string(&self).unwrap();
        println!("the result is : {}", json_string);
        let user_string = format!("{}:{}:{}", self.id, self.name, self.email);
        Response::build()
            .sized_body(json_string.len(), Cursor::new(json_string))
            .header(ContentType::JSON)
            .ok()
    }
}