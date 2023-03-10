fn setup_tracing() {
    let filter_layer =
        EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()));
    let fmt_layer = fmt::layer().json().with_line_number(true).with_thread_names(false);

    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
}
