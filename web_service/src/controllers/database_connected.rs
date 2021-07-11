use deadpool_postgres::{Client, PoolError};
use postgres::{Row, Statement};
use rocket::serde::json::Json;
use rocket::State;

use crate::core::database::db_pool::DbPool;
use crate::model::status_message::StatusMessage;
use crate::model::user::TABLE_NAME_USER;
use crate::model::user::User;

#[get("/getUsersFromDatabase")]
pub async fn get_all_user(db_pool: &State<DbPool>) -> Result<Json<Vec<User>>, Json<StatusMessage>> {
    let client: Result<Client, PoolError> = db_pool.pool.get().await;

    let client: Client = match client {
        Ok(client_positive) => client_positive,
        Err(pool_error) => {
            println!("we are getting an error {}", pool_error);
            return Err(
                Json(
                    StatusMessage {
                        code: 400,
                        message: "this is a custom message".to_string(),
                    }
                )
            );
        }
    };

    let statement: Statement = client
        .prepare_cached(
            &format!("SELECT * FROM {}", TABLE_NAME_USER)
        )
        .await
        .unwrap();

    let result_for_users: Vec<Row> = client.query(&statement, &[]).await.unwrap();

    let mut users = vec![];

    for row in result_for_users {
        let id: i64 = row.get(0);
        let name: String = row.get(1);
        let email: String = row.get(2);

        let user = User {
            id,
            name,
            email,
        };

        users.push(user);
    }

    Ok(
        Json(
            users
        )
    )
}