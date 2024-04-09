use mavio::{protocol::V2, AsyncReceiver};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};

use crate::core::communication::SerpeDialectSender;

pub async fn run_communication_server(
    _communication_message_sender: SerpeDialectSender,
    token: tokio_util::sync::CancellationToken,
) {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).await.unwrap();

    tokio::spawn(async move {
        loop {
            if token.is_cancelled() {
                break;
            }

            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(handle_connection(stream));
                }
                Err(e) => {
                    tracing::error!("Error accepting connection: {:?}", e);
                }
            }
        }
    });
}

async fn handle_connection(stream: TcpStream) {
    let (_reader, _writer) = stream.into_split();

    let listen_handle = tokio::spawn(_listen(_reader));
    let write_handle = tokio::spawn(_write(_writer));

    let _ = tokio::join!(listen_handle, write_handle);
}

async fn _listen<R: AsyncRead + Unpin>(reader: R) -> mavio::error::Result<()> {
    let mut _receiver = AsyncReceiver::versioned(reader, V2);

    Ok(())
}

async fn _write<W: AsyncWrite + Unpin>(_writer: W) {}
