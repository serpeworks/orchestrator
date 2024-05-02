use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query, ResMut, Resource},
};
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    core::domain::{SessionBundle, SessionInformation, SessionStatus},
    dialects::{self, serpe_dialect, SerpeDialect},
};

use super::domain::AgentID;

#[cfg(test)]
mod tests;

pub enum CommsMessage {
    Register {
        agent_id: AgentID,
        sender: SerpeDialectSender,
        receiver: SerpeDialectReceiver,
    },
}

pub type CommsMessageReceiver = tokio::sync::mpsc::Receiver<CommsMessage>;
pub type CommsMessageSender = tokio::sync::mpsc::Sender<CommsMessage>;
pub type SerpeDialectReceiver = tokio::sync::mpsc::Receiver<SerpeDialect>;
pub type SerpeDialectSender = tokio::sync::mpsc::Sender<SerpeDialect>;

pub type SystemID = u8;

#[derive(Clone, Copy)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Component)]
pub struct SessionConnection {
    pub system_id: SystemID,
    pub status: ConnectionStatus,
    incoming_receiver: SerpeDialectReceiver,
    outgoing_sender: SerpeDialectSender,
}

#[derive(Resource)]
pub struct CommunicationResource {
    receiver: CommsMessageReceiver,
}

impl CommunicationResource {
    pub fn new(receiver: CommsMessageReceiver) -> Self {
        CommunicationResource { receiver }
    }
}

pub fn system_communication_general(
    mut resource: ResMut<CommunicationResource>,
    mut commands: Commands,
) {
    // Handle general sender messages
    while let Ok(message) = resource.receiver.try_recv() {
        match message {
            CommsMessage::Register {
                agent_id,
                receiver,
                sender,
            } => {
                let assigned_system_id = 1;

                // Send back a registration ack
                let msg = dialects::serpe_dialect::messages::RegisterAck {
                    system_id: assigned_system_id,
                };

                if let Err(err) = sender.try_send(SerpeDialect::RegisterAck(msg)) {
                    tracing::error!("Failed to send registration ack: {:?}", err);
                    continue;
                }

                let _ = commands.spawn(SessionBundle {
                    session: SessionInformation {
                        agent_id,
                        session_id: 0,
                        session_status: SessionStatus::default(),
                    },
                    sockets: SessionConnection {
                        system_id: assigned_system_id,
                        incoming_receiver: receiver,
                        outgoing_sender: sender.clone(),
                        status: ConnectionStatus::Connected,
                    },
                });
            }
        }
    }
}

pub fn system_communication_receive_messages(
    mut commands: Commands,
    mut query: Query<(Entity, &SessionInformation, &mut SessionConnection)>,
) {
    for (entity, _session, mut connection) in query.iter_mut() {
        loop {
            match connection.incoming_receiver.try_recv() {
                Ok(msg) => match msg {
                    SerpeDialect::Unregister(_) => {
                        commands.entity(entity).despawn();
                        let msg = serpe_dialect::messages::UnregisterAck {};
                        tracing::error!("Unregistering agent");
                        connection
                            .outgoing_sender
                            .try_send(SerpeDialect::UnregisterAck(msg))
                            .unwrap();
                    }
                    _ => {
                        tracing::info!("Other message ignored")
                    }
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    connection.status = ConnectionStatus::Disconnected;
                    break;
                }
            }
        }
    }
}
