use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::rolling;
use std::io::stdout;

pub fn init_tracing() -> WorkerGuard {
    // Create a rolling file appender – rotates daily
    let file_appender = rolling::daily("logs", "ems.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Set up subscriber with both console and file writers
    let console_layer = fmt::layer().with_writer(stdout);
    let file_layer = fmt::layer().with_writer(non_blocking).with_ansi(false); // no ANSI colours in file

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(console_layer)
        .with(file_layer)
        .init();

    guard // must be stored to keep the writer alive
}
