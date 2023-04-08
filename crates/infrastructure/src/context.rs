use std::sync::Arc;

use anyhow::Result;
use sea_orm::DatabaseConnection;

use crate::{config::Config, rdbms};

/// Dependencies needed by the resolvers
#[derive(Debug, Clone)]
pub struct AppContext {
	/// application config instance
	pub config: &'static Config,

	/// The database connections
	pub db: Arc<DatabaseConnection>,
}

/// Intialize dependencies
impl AppContext {
	/// Create a new set of dependencies based on the given shared resources
	pub async fn init(config: &'static Config) -> Result<Self> {
		let dbconn = rdbms::open(&config.database).await;

		let db = Arc::new(dbconn);

		Ok(Self { config, db })
	}
}
