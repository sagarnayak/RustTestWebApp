use crate::contracts::user_contracts::UserContracts;
use crate::model::user::User;
use rocket::State;
use crate::database::db_pool::DbPool;
use rocket::serde::json::Json;
use crate::model::status_message::StatusMessage;

#[async_trait]
impl UserContracts for User{
    async fn fetch_all_user(db_pool: &State<DbPool>) -> Result<Json<Vec<User>>, Json<StatusMessage>> {
        todo!()
    }
}