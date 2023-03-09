use slog::{b, Level};

/// A slog powered log backend.
///
/// Basically the same as `slog-stdlog` but use `slog-global::borrow_global()`.
struct SlogBackend;

fn log_to_slog_level(level: log::Level) -> Level {
    match level {
        log::Level::Trace => Level::Trace,
        log::Level::Debug => Level::Debug,
        log::Level::Info => Level::Info,
        log::Level::Warn => Level::Warning,
        log::Level::Error => Level::Error,
    }
}

fn slog_to_log_level(level: Level) -> log::Level {
    match level {
        Level::Critical | Level::Error => log::Level::Error,
        Level::Warning => log::Level::Warn,
        Level::Debug => log::Level::Debug,
        Level::Trace => log::Level::Trace,
        Level::Info => log::Level::Info,
    }
}

impl log::Log for SlogBackend {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, r: &log::Record) {
        let level = log_to_slog_level(r.metadata().level());

        let args = r.args();
        let target = r.target();
        let module = r.module_path_static().unwrap_or("");
        let file = r.file_static().unwrap_or("");
        let line = r.line().unwrap_or(0);

        let s = slog::RecordStatic {
            location: &slog::RecordLocation {
                file,
                line,
                column: 0,
                function: "",
                module,
            },
            level,
            tag: target,
        };
        crate::borrow_global().log(&slog::Record::new(&s, args, b!()))
    }

    fn flush(&self) {}
}

/// Starts redirecting all logs from the `log` crate `slog_global::borrow_global()`.
///
/// Logs will be always outputted to the active global logger at the time of logging
/// (instead of the global logger when this function is called).
///
/// Basically this function should be called only once.
pub fn redirect_std_log(level: Option<Level>) -> Result<(), log::SetLoggerError> {
    if let Some(level) = level {
        log::set_max_level(slog_to_log_level(level).to_level_filter());
    }

    log::set_logger(&SlogBackend)
}
