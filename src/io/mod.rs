/// IO Module for the drones server.
///

mod diagnostic;

use tracing::info;
use crate::{io::diagnostic::run_diagnostic_server, core::systems::diagnostic::messages::DiagnosticMessageSender};

const DIAGNOSTIC_PORT: u16 = 8080;

pub async fn start_io_task(
    tx: DiagnosticMessageSender,
    token: tokio_util::sync::CancellationToken,
) -> Result<(), ()> {

    info!("IO initialized.");

    run_diagnostic_server(
        tx,
        DIAGNOSTIC_PORT,
        token.clone()
    ).await;

    info!("IO shutting down.");

    return Ok(());
}

