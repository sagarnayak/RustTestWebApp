use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

pub const TABLE_NAME_USER: &str = "users";