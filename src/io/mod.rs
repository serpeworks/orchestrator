mod communication;
/// IO Module for the drones server.
///
mod diagnostic;

use crate::{
    core::{communication::CommunicationPipes, diagnostic::messages::DiagnosticMessageSender},
    io::communication::listen_for_messages,
};

use self::diagnostic::run_diagnostic_server;

const DIAGNOSTIC_PORT: u16 = 8080;

pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken,
    diagnostic_message_sender: DiagnosticMessageSender,
    _communication_message_sender: tokio::sync::mpsc::Sender<CommunicationPipes>,
) -> Result<(), ()> {
    let _ = tokio::join!(
        run_diagnostic_server(diagnostic_message_sender, DIAGNOSTIC_PORT, token.clone()),
        listen_for_messages(_communication_message_sender, token.clone())
    );

    Ok(())
}
