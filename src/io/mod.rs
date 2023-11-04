use tracing::info;

/// IO Module for the drones server.
///
///

#[tracing::instrument]
pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken
) -> Result<(), ()> {

    info!("IO initialized.");

    loop {
        if token.is_cancelled() {
            break;
        }

        tokio::task::yield_now().await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    info!("IO shutting down.");

    return Ok(());
}

