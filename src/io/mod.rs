mod communication;
/// IO Module for the drones server.
///
mod diagnostic;

use crate::{
    core::{communication::SerpeDialectSender, diagnostic::messages::DiagnosticMessageSender},
    io::communication::run_communication_server,
};

use self::diagnostic::run_diagnostic_server;

const DIAGNOSTIC_PORT: u16 = 8080;

pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken,
    diagnostic_message_sender: DiagnosticMessageSender,
    _communication_message_sender: SerpeDialectSender,
) -> Result<(), ()> {
    let _ = tokio::join!(
        run_diagnostic_server(diagnostic_message_sender, DIAGNOSTIC_PORT, token.clone()),
        run_communication_server(_communication_message_sender, token.clone())
    );

    Ok(())
}
