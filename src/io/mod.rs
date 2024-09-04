mod communication;
/// IO Module for the drones server.
///
mod diagnostic;

use crate::{
    config::Configuration,
    core::{communication::CommsMessageSender, diagnostic::messages::DiagnosticMessageSender},
    io::communication::run_communication_server,
};

use self::diagnostic::run_diagnostic_server;

pub async fn start_io_task(
    diagnostic_message_sender: DiagnosticMessageSender,
    communication_message_sender: CommsMessageSender,
    config: &Configuration,
    token: tokio_util::sync::CancellationToken,
) -> Result<(), ()> {
    let _ = tokio::join!(
        run_diagnostic_server(
            diagnostic_message_sender,
            config.diagnostic.clone(),
            token.clone(),
        ),
        run_communication_server(
            communication_message_sender,
            config.communication.port,
            token.clone()
        )
    );

    Ok(())
}
