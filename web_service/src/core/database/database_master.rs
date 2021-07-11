use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use crate::config_controller::ConfigData;
use crate::core::database::db_pool::DbPool;
use crate::core::strings::FAILED_TO_GET_CONFIG_DATA;

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