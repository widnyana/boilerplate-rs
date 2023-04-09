use std::sync::atomic::{AtomicBool, Ordering};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);
}

pub async fn graceful_shutdown_handler() {
	let ctrl_c = async {
		tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let sigterm = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("Failed to install SIGTERM handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let sigterm = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = sigterm => {},
	}

	tracing::info!("Received shutdown signal. Shutting down gracefully...");
	SHUTTING_DOWN.store(true, Ordering::SeqCst);
}
