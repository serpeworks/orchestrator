/// IO Module for the drones server.
///
///

mod diagnostic;

use tracing::info;
use crate::io::diagnostic::run_diagnostic_server;

pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken
) -> Result<(), ()> {

    info!("IO initialized.");

    run_diagnostic_server(8080, token.clone()).await;

    info!("IO shutting down.");

    return Ok(());
}

