use mavio::{protocol::V2, AsyncReceiver, AsyncSender, Endpoint, MavLinkId};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpStream,
};
use tokio_util::sync::CancellationToken;

use crate::{
    core::communication::{
        CommsMessage, CommsMessageSender, SerpeDialectReceiver, SerpeDialectSender,
    },
    dialects::SerpeDialect,
};

const GCS_SYSTEM_ID: u8 = 1;

pub async fn run_communication_server(
    communication_message_sender: CommsMessageSender,
    port: u16,
    token: tokio_util::sync::CancellationToken,
) {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address).await.unwrap();

    let mut connection_tasks = vec![];

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((stream, _)) => {
                        let task = tokio::spawn(handle_connection(
                            stream,
                            communication_message_sender.clone(),
                            token.clone(),
                        ));
                        connection_tasks.push(task);
                    }
                    Err(e) => {
                        tracing::error!("Error accepting connection: {:?}", e);
                    }
                }
            },
            _ = token.cancelled() => {
                break;
            }
        }
    }

    let _ = futures::future::join_all(connection_tasks).await;
}

async fn handle_connection(
    stream: TcpStream,
    communication_message_sender: CommsMessageSender,
    _token: CancellationToken,
) {
    let (reader, writer) = stream.into_split();
    let mut receiver = AsyncReceiver::versioned(reader, V2);
    let sender = AsyncSender::versioned(writer, V2);

    let agent_id = check_registration(&mut receiver).await.unwrap();

    let (incoming_sender, incoming_receiver) = tokio::sync::mpsc::channel(256);
    let (outgoing_sender, outgoing_receiver) = tokio::sync::mpsc::channel(256);

    send_channel_endpoints(
        agent_id,
        communication_message_sender,
        incoming_receiver,
        outgoing_sender,
    )
    .await;

    let listen_handle = tokio::spawn(listen(receiver, incoming_sender));
    let write_handle = tokio::spawn(write(sender, outgoing_receiver));

    let _ = tokio::join!(listen_handle, write_handle);
}

async fn check_registration(receiver: &mut AsyncReceiver<OwnedReadHalf, V2>) -> Result<u32, ()> {
    let frame = receiver.recv().await.unwrap();

    let message = frame.decode::<SerpeDialect>().unwrap();
    if let SerpeDialect::Register(msg) = message {
        Ok(msg.agent_id)
    } else {
        Err(())
    }
}

async fn send_channel_endpoints(
    agent_id: u32,
    communication_message_sender: CommsMessageSender,
    receiver: SerpeDialectReceiver,
    sender: SerpeDialectSender,
) {
    communication_message_sender
        .send(CommsMessage::Register {
            agent_id,
            receiver,
            sender,
        })
        .await
        .unwrap();
}

async fn listen(
    mut receiver: AsyncReceiver<OwnedReadHalf, V2>,
    _incoming_sender: SerpeDialectSender,
) -> mavio::error::Result<()> {
    loop {
        let frame_result = receiver.recv().await;
        if frame_result.is_err() {
            let err = frame_result.err().unwrap();
            match err {
                // TODO: set this to respond on UnexpectedEof
                mavio::error::Error::Io(_error) => {
                    tracing::info!("Connection closed by client.");
                    return Ok(());
                }
                _ => {
                    tracing::error!("Error receiving frame: {:?}", err);
                }
            }
            tracing::error!("Error receiving frame: {:?}", err);
            return Err(err)?;
        }

        let frame = frame_result.unwrap();

        if let Err(err) = frame.validate_checksum::<SerpeDialect>() {
            tracing::error!("Invalid frame: {:?}", err);
            continue;
        }

        // TODO: send to incoming_sender
    }
}

async fn write(
    mut sender: AsyncSender<OwnedWriteHalf, V2>,
    mut outgoing_receiver: SerpeDialectReceiver,
) {
    let endpoint = Endpoint::v2(MavLinkId::new(GCS_SYSTEM_ID, 0));

    loop {
        match outgoing_receiver.recv().await {
            Some(msg) => {
                tracing::info!("Received message to send.");
                let frame = match msg {
                    SerpeDialect::UnregisterAck(msg) => endpoint.next_frame(&msg).unwrap(),
                    SerpeDialect::RegisterAck(msg) => endpoint.next_frame(&msg).unwrap(),
                    _ => {
                        continue;
                    }
                };

                if let Err(err) = sender.send(&frame).await {
                    tracing::error!("Error sending frame: {:?}", err);
                    continue;
                }
            }
            None => {
                tracing::info!("Channel closed.");
                break;
            }
        }
    }
}
