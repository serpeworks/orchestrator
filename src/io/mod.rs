/// IO Module for the drones server.
///

mod diagnostic;
mod communication;

use crate::{core::diagnostic::messages::DiagnosticMessageSender, io::communication::listen_for_messages};

use self::diagnostic::run_diagnostic_server;

const DIAGNOSTIC_PORT : u16 = 8080;

pub async fn start_io_task(
    token: tokio_util::sync::CancellationToken,
    diagnostic_message_sender : DiagnosticMessageSender,
    _communication_message_sender : tokio::sync::mpsc::Sender<()>,
) -> Result<(), ()> {

    let _ = tokio::join!(
        run_diagnostic_server(
            diagnostic_message_sender,
            DIAGNOSTIC_PORT,
            token.clone()
        ),
        listen_for_messages(token.clone())
    );

    return Ok(());
}

