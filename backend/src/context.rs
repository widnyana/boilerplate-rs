use common::config;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<config::config::Config>,
    // db: PgPool,
}
