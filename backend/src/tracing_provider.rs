use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::{self, TraceLayer};
use tracing::instrument;
use tracing::Level;

use infrastructure::config::LogConfig;

// /// setup logging using slog-rs
// pub fn setup_slog() {
//     let slogger = slog::Logger::root(
//         Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
//         o!("version" => env!("CARGO_PKG_VERSION")),
//     );
//
//     logger::set_global(slogger);
// }

/// convert `&String` into `tracing::Level`
#[instrument]
pub fn get_log_level(lvl: &String) -> Level {
    match lvl.as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => Level::INFO,
    }
}

/// setup logging using tracing
#[instrument]
pub fn setup_tracing_v2(conf: &LogConfig) {
    let mut builder = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .json();

    if !conf.level.is_empty() {
        let log_env = get_log_level(&conf.level);
        println!("log env: {}", log_env);
        builder = builder.with_max_level(log_env);
    }

    let subscriber = builder.finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

/// provide a layer to be consumed by axum router
#[instrument]
pub fn provide_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    // used for dumping log as this level on each events configured in `TraceLayer`
    let tracing_level = Level::INFO;

    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing_level))
        .on_response(trace::DefaultOnResponse::new().level(tracing_level))
        .on_failure(trace::DefaultOnFailure::new().level(tracing_level))
}
