#![forbid(unsafe_code)]

use anyhow::Result;
use infrastructure::config::Config;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

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
        let db = Arc::new(sea_orm::Database::connect(&config.database.url).await?);

        Ok(Self { config, db })
    }
}
