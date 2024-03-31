/// IO Module for the drones server.
///

mod diagnostic;
mod communication;

use crate::core::diagnostic::messages::DiagnosticMessageSender;

use self::diagnostic::run_diagnostic_server;

const DIAGNOSTIC_PORT : u16 = 8080;

pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken,
    diagnostic_message_sender : DiagnosticMessageSender,
) -> Result<(), ()> {

    run_diagnostic_server(
        diagnostic_message_sender,
        DIAGNOSTIC_PORT,
        token.clone()
    ).await;

    return Ok(());
}

