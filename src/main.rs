use tokio_util::sync::CancellationToken;
use tracing::info;

pub mod core;
mod io;

/// Setups the tracing module with a global default subscriber.
/// This heavily ties the server with the tracing module for operation.
fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true)
        .with_thread_ids(true)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");
}

/// Initializes handlers for termination signals.
/// Currently it only catches the SIGINT unix signal,
/// TODO: but further consideration should be made on handling SIGTERM signals aswell.
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


#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    use tokio::sync::mpsc;

    use self::core::configuration::Configuration;
    setup_tracing();

    let token = tokio_util::sync::CancellationToken::new();
    setup_signal_handlers(token.clone()).await;

    let config = Configuration {
        frequency: 20, 
    };

    let (tx, rx) = mpsc::channel(256);

    let _ = tokio::join!(
        core::start_core_task(
            rx,
            token.clone(), 
            config
        ),
        io::start_io_task(
            tx,
            token.clone(),
        )
    );
}

