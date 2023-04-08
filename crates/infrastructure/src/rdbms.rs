use std::time::Duration;

use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use tracing::{debug, info, instrument};

use crate::config::DatabaseConfig;

pub mod traits;

#[instrument]
pub async fn open(conf: &DatabaseConfig) -> DatabaseConnection {
	let mut opt = ConnectOptions::new(conf.dsn.clone());

	opt.max_connections(conf.pool.max.unwrap_or(10u32).to_owned()).min_connections(conf.pool.min.unwrap_or(5u32).to_owned()).sqlx_logging(conf.debug);

	if let Some(connect_timeout) = conf.pool.connect_timeout {
		debug!("configuring connect timeout: {} second", connect_timeout);
		opt.connect_timeout(Duration::from_secs(connect_timeout));
	}

	if let Some(idle_timeout) = conf.pool.idle_timeout {
		debug!("configuring idle timeout: {} second", idle_timeout);
		opt.idle_timeout(Duration::from_secs(idle_timeout));
	}

	let db = Database::connect(opt).await.expect("failed to open Database connection");
	info!("Database connected");
	db
}
