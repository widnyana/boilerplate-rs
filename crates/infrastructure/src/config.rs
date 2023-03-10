use std::env;

use anyhow::Result;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// the default `Config` instance
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("unable to retrieve config"));

/// Database pool config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbPool {
    /// Database pool min
    pub min: Option<u32>,
    /// Database pool max
    pub max: Option<u32>,

    /// timeout when performing connect
    pub connect_timeout: Option<u64>,

    /// timeout for idle connection (in second)
    pub idle_timeout: Option<u64>,
}

/// Database config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Database name
    pub name: String,
    /// Full database url
    pub url: String,
    /// Database debug logging
    pub debug: bool,
    /// Database pool config
    pub pool: DbPool,
}

/// Logging a.k.a Tracing config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

/// Application config
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    // application run mode, production or development
    pub run_mode: String,

    // port to bind
    pub port: u16,

    /// database connection configuration
    pub database: DatabaseConfig,

    /// logging configuration
    pub log: LogConfig,
}

impl Config {
    /// Create a new Config by merging in various sources
    pub fn new() -> Result<Self> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());

        let config = Figment::new()
            // load default
            .merge(Yaml::file("config/default.yaml"))
            // Load local overrides
            .merge(Yaml::file("config/local.yaml"))
            // Load run mode overrides
            .merge(Yaml::file(format!("config/{}.toml", run_mode)))
            // Load environment variables
            .merge(
                // Support the nested structure of the config manually
                Env::raw()
                    // Split the Database variables
                    .map(|key| {
                        key.as_str()
                            .replace("DATABASE_POOL_", "DATABASE.POOL.")
                            .into()
                    })
                    .map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into())
                    // Split the Redis variables
                    .map(|key| key.as_str().replace("REDIS_", "REDIS.").into())
                    // Split the Auth variables
                    .map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
                    .map(|key| key.as_str().replace("AUTH_", "AUTH.").into()),
            )
            // Serialize and freeze
            .extract()?;

        Ok(config)
    }
    /// Return true if the `run_mode` is "development"
    pub fn is_dev(&self) -> bool {
        self.run_mode == "development"
    }
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
    &CONFIG
}
