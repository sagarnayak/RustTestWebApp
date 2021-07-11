use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusMessage {
    pub code: i32,
    pub message: String,
}