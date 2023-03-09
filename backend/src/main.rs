#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]
// #![allow(unused_imports,missing_docs,unused_variables, dead_code, unused_macros)]
// #![deny(unsafe_code)]

use std::process::ExitCode;
use std::sync::Mutex;

use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::{Router, Server};
use slog::{o, Drain};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use infrastructure::env;

use crate::state::AppState;

mod app;
mod shared;
mod state;

fn setup_logging() {
    // logging
    let slogger = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!("version" => env!("CARGO_PKG_VERSION")),
    );

    logger::set_global(slogger);
}

fn setup_tracing() {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    let fmt_layer = fmt::layer().json().with_target(false);

    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
}

pub(crate) fn make_app(max_body_size: usize) -> Router<AppState> {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(DefaultBodyLimit::max(max_body_size))
                .layer(DefaultBodyLimit::disable())
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http()), // .layer(TimeoutLayer::new(env::HTTP_TIMEOUT)),
        )
        .layer(cors)
        .fallback(shared::handle_404_json)
}

async fn start() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    setup_tracing();

    let addr = env::addr()?;
    let max_body_size = env::max_body_size()?;

    let state = state::AppState {};
    let service: Router<()> = make_app(max_body_size).with_state(state);

    tracing::info!("serving on {addr}");
    tracing::info!("restricting maximum body size to {max_body_size} bytes");

    Server::bind(&addr)
        .serve(service.into_make_service())
        .with_graceful_shutdown(shared::shutdown_signal())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match start().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
