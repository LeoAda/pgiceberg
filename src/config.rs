use std::env;
use std::fs;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct Config {
    pub pg_connection_string: String,
}

impl Config {
    pub fn from_env() -> Self {
        let read_env = |key: &str| -> String {
            env::var(key).unwrap_or_else(|e| {
                error!("Error reading environment variable {}: {}", key, e);
                std::process::exit(1);
            })
        };

        Config {
            pg_connection_string: read_env("POSTGRES_CONNECTION_STRING"),
        }
    }

    pub fn from_file() -> Self {
        let read_secret = |path: &str| -> String {
            fs::read_to_string(path).unwrap_or_else(|e| {
                error!("Error reading config file {}: {}", path, e);
                std::process::exit(1);
            })
        };
        let pg_username = read_secret("/run/secrets/postgres/username");
        let pg_password = read_secret("/run/secrets/postgres/password");
        let pg_host = read_secret("/config/PG_HOST");
        let pg_port = read_secret("/config/PG_PORT");
        let pg_database: String = read_secret("/run/secrets/postgres/database");
        let pg_connection_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            pg_username.trim(),
            pg_password.trim(),
            pg_host.trim(),
            pg_port.trim(),
            pg_database.trim()
        );
        Config {
            pg_connection_string: pg_connection_string,
        }
    }

    pub fn from_env_or_file(debug: Option<bool>) -> Self {
        if debug.unwrap_or(false) {
            info!("Loading configuration from environment variables...");
            Self::from_env()
        } else {
            info!("Loading configuration from file...");
            Self::from_file()
        }
    }
}