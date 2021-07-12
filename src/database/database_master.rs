use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Client, PoolError};
use tokio_postgres::NoTls;

use crate::config_controller::ConfigData;
use crate::database::db_pool::DbPool;
use crate::core::strings::FAILED_TO_GET_CONFIG_DATA;
use crate::model::status_message::StatusMessage;
use rocket::serde::json::Json;
use rocket::State;

fn get_pool() -> Pool {
    let config = ConfigData::new().expect(FAILED_TO_GET_CONFIG_DATA);
    let database = config.database;

    let host = database.host;
    let port = database.port;
    let user = database.user;
    let password = database.password;
    let database_name = database.database_name;

    let mut cfg = Config::new();
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.dbname = Some(database_name);
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    cfg.create_pool(NoTls).unwrap()
}

pub fn get_db_pools() -> DbPool {
    DbPool {
        pool: get_pool()
    }
}

pub async fn resolve_client (db_pool: &State<DbPool>)->Result< Client, Json<StatusMessage>>{
    let client: Result<Client, PoolError> = db_pool.pool.get().await;

    let client: Client =  match client {
        Ok(client_positive) => client_positive,
        Err(pool_error) => {
            println!("we are getting an error {}", pool_error);
            return Err(
                Json(
                    StatusMessage {
                        code: 400,
                        message: pool_error.to_string(),
                    }
                )
            );
        }
    };

    Ok(client)
}