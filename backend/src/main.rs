#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]
#![deny(rust_2018_idioms)]
#![allow(dead_code)]
// #![allow(unused_imports,missing_docs,unused_variables, dead_code, unused_macros)]

use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use axum::{extract::Extension, http::Method, routing::get, Router, Server};
use slog::{o, Drain};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::RequestDecompressionLayer,
    trace::{self, TraceLayer},
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use infrastructure::config::{get_config, Config};

use crate::routes::{handle_404_json, health_handler};
use crate::state::AppContext;

mod routes;
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
    let filter_layer = EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()));
    let fmt_layer = fmt::layer().json().with_line_number(true).with_thread_names(false);

    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
}

pub(crate) fn make_app(ctx: Arc<AppContext>) -> Router {
    let mut tracing_level = tracing::Level::ERROR;
    if ctx.config.run_mode.to_lowercase() != "production" {
        tracing_level = tracing::Level::INFO;
    }

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing_level))
                .on_response(trace::DefaultOnResponse::new().level(tracing_level))
                .on_failure(trace::DefaultOnFailure::new().level(tracing_level)),
        )
        .layer(CompressionLayer::new())
        .layer(Extension(ctx))
        .layer(cors)
        .layer(Extension(RequestDecompressionLayer::new()))
        .fallback(handle_404_json)
}

async fn start(config: &'static Config) -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();
    setup_tracing();

    let addr = &format!("0.0.0.0:{}", config.port)
        .parse()
        .expect("Unable to parse bind address");

    let ctx: Arc<AppContext> = Arc::new(AppContext::init(config).await?);
    let service: Router<()> = make_app(ctx);

    tracing::info!("serving on {addr}");

    Server::bind(&addr)
        .serve(service.into_make_service())
        .with_graceful_shutdown(shared::shutdown_signal())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let config = get_config();

    match start(config).await {
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
