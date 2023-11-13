use tokio_util::sync::CancellationToken;
use tracing::info;

mod core;
mod io;

/// Setups the tracing module with a global default subscriber.
/// This heavily ties the server with the tracing module for operation.
fn setup_tracing() {

    let subscriber = tracing_subscriber::fmt()
        .with_line_number(true)
        .compact()
        .with_thread_ids(true)
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
    use self::core::configuration::Configuration;

    setup_tracing();

    let token = tokio_util::sync::CancellationToken::new();
    setup_signal_handlers(token.clone()).await;

    let config = Configuration {
        frequency: 200, 
    };

    let core_token = token.clone();
    let core_handle = tokio::spawn(async move {
        return core::start_core_task(core_token, config).await;
    });

    let io_token = token.clone();
    let io_handle = tokio::spawn(async move {
        return io::start_io_task(io_token).await;
    });

    let _ = tokio::try_join!(
        core_handle, io_handle
    );
}

