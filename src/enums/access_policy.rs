use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AccessPolicy {
    Admin,
    Merchant,
    FrontDesk,
}