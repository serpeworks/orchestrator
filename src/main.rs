mod mavlink {
    include!(concat!(env!("OUT_DIR"), "/mavlink/mod.rs"));
}

mod config;
pub mod core;
mod io;
mod utils;

pub use mavlink::dialects;

const DIAGNOSTIC_CHANNEL_SIZE: usize = 256;
const COMMUNICATION_CHANNEL_SIZE: usize = 256;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    use utils::{load_perimeter, setup_signal_handlers, setup_tracing};

    setup_tracing();
    let config = config::load_config().expect("Failed to load configuration");

    let token = tokio_util::sync::CancellationToken::new();
    setup_signal_handlers(token.clone()).await;

    let (diagnostic_tx, diagnostic_rx) = tokio::sync::mpsc::channel(DIAGNOSTIC_CHANNEL_SIZE);
    let (communication_incoming_tx, communication_incoming_rx) =
        tokio::sync::mpsc::channel(COMMUNICATION_CHANNEL_SIZE);

    let kml = load_perimeter(config.core.perimeter_filepath.clone())
        .expect("An invalid perimeter file path was used!");

    let _ = tokio::join!(
        io::start_io_task(
            diagnostic_tx,
            communication_incoming_tx,
            &config,
            token.clone(),
        ),
        core::start_core_task(
            &config,
            diagnostic_rx,
            communication_incoming_rx,
            token.clone(),
            kml,
        ),
    );
}
