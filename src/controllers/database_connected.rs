use rocket::serde::json::Json;
use rocket::State;

use crate::contracts::user_contracts::UserContracts;
use crate::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/getUsersFromDatabase")]
pub async fn get_all_user(db_pool: &State<DbPool>) -> Result<Json<Vec<User>>, Json<StatusMessage>> {
    User::fetch_all_user(db_pool).await
}