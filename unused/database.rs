use std::time::Duration;

use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn config_db(url: &str, pool_size: u32) -> Pool {
    tracing::info!("Migrating and configurating database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(10))
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
