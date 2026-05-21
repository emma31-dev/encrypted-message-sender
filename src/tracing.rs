use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_tracing() {
    // Set up console subscriber with environment filter (RUST_LOG)
    tracing_subscriber::registry()
        .with(fmt::layer()) // human‑readable output
        .with(EnvFilter::from_default_env()) // read from RUST_LOG env var
        .init();
}
