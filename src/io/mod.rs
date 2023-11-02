use tracing::info;
use tracing_attributes::instrument;

/// IO Module for the drones server.
///
///

#[instrument]
pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken
) -> Result<(), ()> {

    info!("IO initialized.");

    loop {

        if token.is_cancelled() {
            break;
        }

        tokio::task::yield_now().await;
    }

    info!("IO shutting down.");

    return Ok(());
}

