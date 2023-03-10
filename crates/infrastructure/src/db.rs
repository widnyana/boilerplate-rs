use crate::config::DatabaseConfig;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use std::time::Duration;
use tracing::{info, instrument};

#[instrument]
pub async fn open(conf: &DatabaseConfig) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(conf.url.clone());

    opt.max_connections(conf.pool.max.unwrap_or(10u32).to_owned())
        .min_connections(conf.pool.min.unwrap_or(5u32).to_owned())
        .connect_timeout(Duration::from_secs(conf.pool.connect_timeout.unwrap_or(5u64)))
        .idle_timeout(Duration::from_secs(conf.pool.idle_timeout.unwrap_or(60u64)))
        .sqlx_logging(conf.debug);

    let db = Database::connect(opt)
        .await
        .expect("failed to open Database connection");

    info!("Database connected");
    db
}
