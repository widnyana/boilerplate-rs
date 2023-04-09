use tower_http::{
	classify::{ServerErrorsAsFailures, SharedClassifier},
	trace as TowerTrace,
	trace::TraceLayer,
};
use tracing::{instrument, log::debug, Level};

use crate::config::Config;

pub mod otel;
pub mod otel_spans;

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
#[instrument(skip(conf))]
pub fn setup_tracing(conf: &Config) {
	let mut builder = tracing_subscriber::fmt()
		.log_internal_errors(false)
		.with_ansi(false)
		.with_file(true)
		.with_level(true)
		.with_line_number(true)
		.with_target(true)
		.json();

	if !conf.log.level.is_empty() {
		let log_env = get_log_level(&conf.log.level);
		debug!("log env: {}", log_env);
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
		.make_span_with(TowerTrace::DefaultMakeSpan::new().level(tracing_level))
		.on_response(TowerTrace::DefaultOnResponse::new().level(tracing_level))
		.on_failure(TowerTrace::DefaultOnFailure::new().level(tracing_level))
}
