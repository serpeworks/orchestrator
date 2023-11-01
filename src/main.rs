use tokio_util::sync::CancellationToken;

mod core;
mod io;

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
        println!("Ctrl-C Signal received! Gracefully shutting down.");
        token.cancel();
        println!("Shut down finalized.");
    });
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {

    let token = tokio_util::sync::CancellationToken::new();
    setup_signal_handlers(token.clone()).await;

    let _ = tokio::try_join!(
        core::start_core_task(
            token.clone()
        ),
        io::start_io_task(
            token.clone()
        )
    );

    println!("Shutting down!");

}

