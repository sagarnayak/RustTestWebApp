use rocket::serde::json::Json;
use rocket::State;

use crate::database::db_pool::DbPool;
use crate::database::user::fetch_all_user;
use crate::model::status_message::StatusMessage;
use crate::model::user::User;

#[get("/getUsersFromDatabase")]
pub async fn get_all_user(db_pool: &State<DbPool>) -> Result<Json<Vec<User>>, Json<StatusMessage>> {
    fetch_all_user(db_pool).await
}