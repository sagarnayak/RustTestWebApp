use std::fmt;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JWTConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigData {
    pub database: DatabaseConfig,
    pub jwt: JWTConfig,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

const CONFIG_FILE_PATH_TWO: &str = "/home/ubuntu/rust_configs/config/Default.toml";
const CONFIG_FILE_PREFIX_TWO: &str = "/home/ubuntu/rust_configs/config/";

impl ConfigData {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        println!("the environment selected is {}", &env);

        let mut s = Config::new();
        let s = match s.set("env", env.clone()) {
            Ok(positive) => positive,
            Err(error) => {
                println!("getting an error at the env cloning ..{}", error.to_string().clone());
                return Err(ConfigError::NotFound(error.to_string()));
            }
        };

        let s = match s.merge(File::with_name(CONFIG_FILE_PATH)) {
            Ok(positive) => positive,
            Err(error) => {
                println!("getting an error at CONFIG_FILE_PATH insertion {}. trying to insert alternative path.", error.to_string());
                match s.merge(File::with_name(CONFIG_FILE_PATH_TWO)) {
                    Ok(positive_two) => positive_two,
                    Err(error_two) => {
                        println!("getting an error at CONFIG_FILE_PATH_TWO insertion {}", error_two.to_string().clone());
                        return Err(ConfigError::NotFound(error_two.to_string()));
                    }
                }
            }
        };

        let s = match  s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env))) {
            Ok(positive) => positive,
            Err(error) => {
                println!("getting an error at CONFIG_FILE_PREFIX insertion {}. trying to insert alternative path.", error.to_string());
                match  s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX_TWO, env))) {
                    Ok(positive_two) => positive_two,
                    Err(error_two) => {
                        println!("getting an error at CONFIG_FILE_PREFIX_TWO insertion {}", error_two.to_string().clone());
                        return Err(ConfigError::NotFound(error_two.to_string()));
                    }
                }
            }
        };

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("ea").separator("__"))?;

        s.clone().try_into()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}