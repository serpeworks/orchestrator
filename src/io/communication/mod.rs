
use mavio::{protocol::V2, AsyncReceiver};
use tokio::{io::{AsyncRead, AsyncWrite}, net::{TcpListener, TcpSocket, TcpStream}};

use crate::mavlink;


pub async fn listen_for_messages(
    token: tokio_util::sync::CancellationToken, 
) {
    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).await.unwrap();

    tokio::spawn(async move {
        loop {
            if token.is_cancelled() {
                break
            }

            let (stream, _) = listener.accept().await.unwrap();

            launch_communication_tasks(stream).await;
        }
    });
}

async fn launch_communication_tasks(
    stream: TcpStream 
) {
    let (reader, writer) = stream.into_split();

    tokio::spawn(listen(reader));
    tokio::spawn(write_to(writer));
}

async fn listen<R: AsyncRead + Unpin>(
    reader: R,
) -> mavio::error::Result<()> {
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

async fn write_to<W: AsyncWrite + Unpin>(
    writer: W,
) {
}


