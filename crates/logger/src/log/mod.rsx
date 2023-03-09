#[cfg(test)]
mod drain;
mod global;

pub use slog::{crit, debug, error, info, trace, warn};

#[macro_export]
/// Log a trace level message if the given logger is `Some`, otherwise do nothing.
macro_rules! l_trace(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::trace!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::trace!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a debug level message if the given logger is `Some`, otherwise do nothing.
macro_rules! l_debug(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::debug!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::debug!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log an info level message if the given logger is `Some`, otherwise do nothing.
macro_rules! l_info(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::info!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::info!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a warn level message if the given logger is `Some`, otherwise do nothing.
macro_rules! l_warn(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::warn!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::warn!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log an error level message if the given logger is `Some`, otherwise do nothing.
macro_rules! l_error(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::error!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::error!(log, $($args)+);
        }
    };
);

#[macro_export]
/// Log a crit level message if the given logger is `Some`,, otherwise do nothing.
macro_rules! l_crit(
    ($l:expr, #$tag:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::crit!(log, #$tag, $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            $crate::crit!(log, $($args)+);
        }
    };
);

#[cfg(test)]
mod tests {
    use crate::drain::Buffer;
    use slog::{o, Logger};

    #[test]
    fn trace() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        l_trace!(opt_logger, "SUCCESS: trace");
        l_trace!(opt_logger, #"tag", "SUCCESS: trace with tag");
        opt_logger = None;
        l_trace!(opt_logger, "FAILED: trace");
        l_trace!(opt_logger, #"tag", "FAILED: trace with tag");
        assert_eq!("SUCCESS: traceSUCCESS: trace with tag", format!("{}", buffer));
    }

    #[test]
    fn debug() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        l_debug!(opt_logger, "SUCCESS: debug");
        l_debug!(opt_logger, #"tag", "SUCCESS: debug with tag");
        opt_logger = None;
        l_debug!(opt_logger, "FAILED: debug");
        l_debug!(opt_logger, #"tag", "FAILED: debug with tag");
        assert_eq!("SUCCESS: debugSUCCESS: debug with tag", format!("{}", buffer));
    }

    #[test]
    fn info() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        l_info!(opt_logger, "SUCCESS: info");
        l_info!(opt_logger, #"tag", "SUCCESS: info with tag");
        opt_logger = None;
        l_info!(opt_logger, "FAILED: info");
        l_info!(opt_logger, #"tag", "FAILED: info with tag");
        assert_eq!("SUCCESS: infoSUCCESS: info with tag", format!("{}", buffer));
    }

    #[test]
    fn warn() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        l_warn!(opt_logger, "SUCCESS: warn");
        l_warn!(opt_logger, #"tag", "SUCCESS: warn with tag");
        opt_logger = None;
        l_warn!(opt_logger, "FAILED: warn");
        l_warn!(opt_logger, #"tag", "FAILED: warn with tag");
        assert_eq!("SUCCESS: warnSUCCESS: warn with tag", format!("{}", buffer));
    }

    #[test]
    fn error() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        l_error!(opt_logger, "SUCCESS: error");
        l_error!(opt_logger, #"tag", "SUCCESS: error with tag");
        opt_logger = None;
        l_error!(opt_logger, "FAILED: error");
        l_error!(opt_logger, #"tag", "FAILED: error with tag");
        assert_eq!("SUCCESS: errorSUCCESS: error with tag", format!("{}", buffer));
    }

    #[test]
    fn crit() {
        let buffer = Buffer::default();
        let logger = Logger::root(buffer.clone(), o!());
        let mut opt_logger: Option<Logger> = Some(logger);
        l_crit!(opt_logger, "SUCCESS: crit");
        l_crit!(opt_logger, #"tag", "SUCCESS: crit with tag");
        opt_logger = None;
        l_crit!(opt_logger, "FAILED: crit");
        l_crit!(opt_logger, #"tag", "FAILED: crit with tag");
        assert_eq!("SUCCESS: critSUCCESS: crit with tag", format!("{}", buffer));
    }
}
