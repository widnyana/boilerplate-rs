use arc_swap::{ArcSwap, Guard};
use lazy_static::lazy_static;
use slog::{o, Logger};
use std::sync::Arc;

#[cfg(feature = "log")]
mod redirect;

#[cfg(feature = "log")]
pub use self::redirect::*;

// create a logger that discard everything
fn discard_logger() -> Logger {
    Logger::root(slog::Discard, o!())
}

lazy_static! {
    static ref SLOG_GLOBAL_LOGGER: ArcSwap<Logger> = ArcSwap::from(Arc::new(discard_logger()));
}

// replace global logger with `l`
pub fn set_global(l: slog::Logger) {
    SLOG_GLOBAL_LOGGER.store(Arc::new(l));
}

/// Gets the global `Logger`.
///
/// If you only want to access the global logger temporarily (i.e. as a local variable on stack but
/// not structures), use `borrow_global()` which is more efficient.
pub fn get_global() -> Arc<Logger> {
    SLOG_GLOBAL_LOGGER.load_full()
}

/// Temporary borrows the global `Logger`.
pub fn borrow_global() -> Guard<Arc<Logger>> {
    SLOG_GLOBAL_LOGGER.load()
}

/// Clears the global `Logger` and discard future logging.
pub fn clear_global() {
    SLOG_GLOBAL_LOGGER.store(Arc::new(discard_logger()));
}

/// Logs a critical level message using the global logger.
#[macro_export]
macro_rules! crit( ($($args:tt)+) => {
    ::slog::slog_crit![$crate::borrow_global(), $($args)+]
};);
/// Logs a error level message using the global logger.
#[macro_export]
macro_rules! error( ($($args:tt)+) => {
    ::slog::slog_error![**$crate::borrow_global(), $($args)+]
};);
/// Logs a warning level message using the global logger.
#[macro_export]
macro_rules! warn( ($($args:tt)+) => {
    ::slog::slog_warn![**$crate::borrow_global(), $($args)+]
};);
/// Logs a info level message using the global logger.
#[macro_export]
macro_rules! info( ($($args:tt)+) => {
    ::slog::slog_info![**$crate::borrow_global(), $($args)+]
};);
/// Logs a debug level message using the global logger.
#[macro_export]
macro_rules! debug( ($($args:tt)+) => {
    ::slog::slog_debug![**$crate::borrow_global(), $($args)+]
};);
/// Logs a trace level message using the global logger.
#[macro_export]
macro_rules! trace( ($($args:tt)+) => {
    ::slog::slog_trace![**$crate::borrow_global(), $($args)+]
};);
