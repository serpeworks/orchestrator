use mavio::{protocol::V2, AsyncReceiver, AsyncSender, Endpoint, MavLinkId};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpStream,
};
use tokio_util::sync::CancellationToken;

use crate::{
    core::{
        communication::{
            CommsMessage, CommsMessageSender, SerpeDialectReceiver, SerpeDialectSender,
        },
        domain::AgentID,
        geo::Coordinates,
    },
    dialects::SerpeDialect,
};

const GCS_SYSTEM_ID: u8 = 0;
const CONNECTION_CHANNEL_BUFFER_SIZE: usize = 256;

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

    let result = check_registration(&mut receiver).await;
    if result.is_err() {
        tracing::warn!("Attempted Connection Fail: First message on stream is not registration!");
        return;
    }

    let (agent_id, coordinates) = result.unwrap();

    let (incoming_sender, incoming_receiver) =
        tokio::sync::mpsc::channel(CONNECTION_CHANNEL_BUFFER_SIZE);
    let (outgoing_sender, outgoing_receiver) =
        tokio::sync::mpsc::channel(CONNECTION_CHANNEL_BUFFER_SIZE);

    send_session_endpoints(
        agent_id,
        communication_message_sender,
        incoming_receiver,
        outgoing_sender,
        coordinates,
    )
    .await;

    let listen_handle = tokio::spawn(listen(receiver, incoming_sender));
    let write_handle = tokio::spawn(write(sender, outgoing_receiver));

    let _ = tokio::join!(listen_handle, write_handle);
}

async fn check_registration(
    receiver: &mut AsyncReceiver<OwnedReadHalf, V2>,
) -> Result<(AgentID, Coordinates), ()> {
    let result = receiver.recv().await;

    if result.is_err() {
        return Err(());
    }

    let message = result.unwrap().decode::<SerpeDialect>().unwrap();
    if let SerpeDialect::Register(msg) = message {
        Ok((
            msg.agent_id,
            Coordinates {
                longitude: msg.longitude as f64,
                latitude: msg.latitude as f64,
            },
        ))
    } else {
        Err(())
    }
}

async fn send_session_endpoints(
    agent_id: u32,
    communication_message_sender: CommsMessageSender,
    receiver: SerpeDialectReceiver,
    sender: SerpeDialectSender,
    coordinates: Coordinates,
) {
    communication_message_sender
        .send(CommsMessage::Register {
            agent_id,
            receiver,
            sender,
            coordinates,
        })
        .await
        .unwrap();
}

async fn listen(
    mut receiver: AsyncReceiver<OwnedReadHalf, V2>,
    incoming_sender: SerpeDialectSender,
) -> mavio::error::Result<()> {
    loop {
        let frame_result = receiver.recv().await;

        if let Err(err) = frame_result {
            match err {
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

        if frame_result.is_err() {
            let err = frame_result.unwrap_err();
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

        match frame.decode::<SerpeDialect>() {
            Ok(msg) => {
                if let Err(err) = incoming_sender.send(msg).await {
                    tracing::error!("Could not send message internally: {:?}", err);
                }
            }
            Err(_) => {
                tracing::error!("Error decoding frame.");
                continue;
            }
        }
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
                let frame = match msg {
                    SerpeDialect::RegisterAck(msg) => endpoint.next_frame(&msg).unwrap(),
                    SerpeDialect::UnregisterAck(msg) => endpoint.next_frame(&msg).unwrap(),
                    SerpeDialect::HeartbeatAck(msg) => endpoint.next_frame(&msg).unwrap(),
                    SerpeDialect::MissionRequest(msg) => endpoint.next_frame(&msg).unwrap(),
                    SerpeDialect::MissionAcceptAck(msg) => endpoint.next_frame(&msg).unwrap(),
                    SerpeDialect::MissionFinishedAck(msg) => endpoint.next_frame(&msg).unwrap(),
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
