use mavio::{protocol::V2, AsyncReceiver};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};

use crate::{core::communication::CommunicationPipes, dialects::SerpeDialect, mavlink};

const CHANNEL_SIZE: usize = 256;

pub async fn listen_for_messages(
    _communication_message_sender: tokio::sync::mpsc::Sender<CommunicationPipes>,
    token: tokio_util::sync::CancellationToken,
) {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).await.unwrap();

    tokio::spawn(async move {
        loop {
            if token.is_cancelled() {
                break;
            }

            let (stream, _) = listener.accept().await.unwrap();
            let (_incoming_writer, _incoming_reader) =
                tokio::sync::mpsc::channel::<SerpeDialect>(CHANNEL_SIZE);
            let (_outgoing_writer, _outgoing_reader) =
                tokio::sync::mpsc::channel::<SerpeDialect>(CHANNEL_SIZE);

            launch_communication_tasks(stream).await;
        }
    });
}

async fn launch_communication_tasks(stream: TcpStream) {
    let (reader, writer) = stream.into_split();

    tokio::spawn(listen(reader));
    tokio::spawn(write_to(writer));
}

async fn listen<R: AsyncRead + Unpin>(reader: R) -> mavio::error::Result<()> {
    let mut receiver = AsyncReceiver::versioned(reader, V2);
    loop {
        let frame = receiver.recv().await?;
        tracing::info!("Exiting listen loop");

        match frame.decode() {
            Ok(msg) => {
                if let mavlink::dialects::SerpeDialect::Heartbeat(msg) = msg {
                    tracing::info!("Heartbeat message: {:?}", msg);
                }
            }
            Err(_) => {
                tracing::error!("Failed to decode message");
            }
        }
    }
}

async fn write_to<W: AsyncWrite + Unpin>(_writer: W) {}
