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
	/// Full database url in data source name format
	pub dsn: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tracing {
	/// The OpenTelemetry address to send events to if given.
	pub otel_address: Option<String>,
	/// The ratio at which to sample spans when sending to OpenTelemetry. When
	/// not given it defaults to always sending. If the OpenTelemetry address is
	/// not set, this will do nothing.
	pub otel_sample_ratio: Option<f64>,

	/// Whether to enable the logging of the databases at the configured log
	/// level. This may be useful for analyzing their response times.
	pub db_tracing: bool,
}

/// Application config
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
	/// application run mode, production or development
	pub run_mode: String,

	/// port to bind
	pub port: u16,

	/// database connection configuration
	pub database: DatabaseConfig,

	/// logging configuration
	pub log: LogConfig,

	/// tracing configuration
	pub tracing: Tracing,
}

impl Config {
	/// Create a new Config by merging in various sources
	pub fn new() -> Result<Self> {
		let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());

		let config = Figment::new()
			.merge(Yaml::file("config/default.yaml"))
			.merge(Yaml::file("config/local.yaml"))
			.merge(Yaml::file(format!("config/{}.toml", run_mode)))
			.merge(
				Env::raw()
					.map(|key| key.as_str().replace("DATABASE_POOL_", "DATABASE.POOL.").into())
					.map(|key| key.as_str().replace("DATABASE_", "DATABASE.").into())
					.map(|key| key.as_str().replace("REDIS_", "REDIS.").into())
					.map(|key| key.as_str().replace("AUTH_CLIENT_", "AUTH.CLIENT.").into())
					.map(|key| key.as_str().replace("AUTH_", "AUTH.").into()),
			)
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
