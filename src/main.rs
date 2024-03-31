use tokio_util::sync::CancellationToken;
use tracing::info;

pub mod core;
mod io;

/// Setups the tracing module with a global default subscriber.
/// This heavily ties the server with the tracing module for operation.
fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true) .with_thread_ids(true)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");
}

/// Initializes handlers for termination signals.
/// Currently it only catches the SIGINT unix signal,
async fn setup_signal_handlers(
    token: CancellationToken
) {   
    // for ctrl-c
    #[cfg(unix)]
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl-C Signal received! Gracefully shutting down.");
        token.cancel();
        info!("Shut down finalized.");
    });
}

const DIAGNOSTIC_CHANNEL_SIZE: usize = 256;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {

    setup_tracing();

    let token = tokio_util::sync::CancellationToken::new();
    setup_signal_handlers(token.clone()).await;

    // init channels
    let (tx, rx) = tokio::sync::mpsc::channel(DIAGNOSTIC_CHANNEL_SIZE);

    let _ = tokio::join!(
        core::start_core_task(
            token.clone(),
            rx,
        ),
        io::start_io_task(
            token.clone(),
            tx,
        )
    );
}
