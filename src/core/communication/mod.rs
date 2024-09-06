use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query, Res, ResMut, Resource},
};
use message_pools::{MessageSenderPool, MessageSnapshot};
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    core::{
        domain::{Attitude, SessionBundle, SessionInformation, SessionStatus},
        geo::Coordinates,
    },
    dialects::{self, serpe_dialect, SerpeDialect},
};

use super::{
    domain::{AgentID, SystemID},
    misc::{resource::ConfigurationResource, system_id_table::SystemIdTable},
};

pub mod message_pools;

#[cfg(test)]
mod tests;

pub enum CommsMessage {
    Register {
        agent_id: AgentID,
        coordinates: Coordinates,
        sender: SerpeDialectSender,
        receiver: SerpeDialectReceiver,
    },
}

pub type CommsMessageReceiver = tokio::sync::mpsc::Receiver<CommsMessage>;
pub type CommsMessageSender = tokio::sync::mpsc::Sender<CommsMessage>;
pub type SerpeDialectReceiver = tokio::sync::mpsc::Receiver<SerpeDialect>;
pub type SerpeDialectSender = tokio::sync::mpsc::Sender<SerpeDialect>;

const MAX_MESSAGE_ITERATIONS: u32 = 64;

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
        Self { receiver }
    }
}

pub fn system_communication_general(
    config_resource: Res<ConfigurationResource>,
    mut resource: ResMut<CommunicationResource>,
    mut id_table: ResMut<SystemIdTable>,
    mut commands: Commands,
) {
    while let Ok(message) = resource.receiver.try_recv() {
        match message {
            CommsMessage::Register {
                agent_id,
                receiver,
                sender,
                coordinates,
            } => {
                tracing::warn!("Received registration request from agent_id: {}", agent_id);
                if config_resource.config.max_number_of_drones <= id_table.count() {
                    tracing::warn!(
                        "Ignoring registration request from agent_id:{}, network is full",
                        agent_id
                    );
                    continue;
                }

                let assigned_system_id_opt = id_table.allocate();
                if assigned_system_id_opt.is_none() {
                    continue;
                }

                let system_id =
                    assigned_system_id_opt.expect("Expected a valid assigned system id");
                let msg = dialects::serpe_dialect::messages::RegisterAck { system_id };

                if let Err(err) = sender.try_send(SerpeDialect::RegisterAck(msg)) {
                    tracing::error!("Failed to send registration ack: {:?}", err);
                    id_table.release(system_id);
                    continue;
                }

                let _ = commands.spawn(SessionBundle {
                    session: SessionInformation {
                        agent_id,
                        session_id: 0,
                        session_status: SessionStatus::default(),
                    },
                    attitude: Attitude { coordinates },
                    sockets: SessionConnection {
                        system_id,
                        incoming_receiver: receiver,
                        outgoing_sender: sender.clone(),
                        status: ConnectionStatus::Connected,
                    },
                    snapshot: MessageSnapshot { messages: vec![] },
                    sender_pool: MessageSenderPool { messages: vec![] },
                });
            }
        }
    }
}

pub fn system_communication_receive_messages(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &SessionInformation,
        &mut SessionConnection,
        &mut MessageSnapshot,
    )>,
    mut id_table: ResMut<SystemIdTable>,
) {
    for (_, _, _, mut snapshot) in query.iter_mut() {
        snapshot.clear();
    }

    for (entity, _, mut connection, mut snapshot) in query.iter_mut() {
        for _ in 0..MAX_MESSAGE_ITERATIONS {
            match connection.incoming_receiver.try_recv() {
                Ok(msg) => match msg {
                    SerpeDialect::Unregister(_) => {
                        tracing::info!("Unregister requested by {}", connection.system_id);

                        commands.entity(entity).despawn();

                        id_table.release(connection.system_id);
                        let msg = serpe_dialect::messages::UnregisterAck {};

                        let _ = connection
                            .outgoing_sender
                            .try_send(SerpeDialect::UnregisterAck(msg));

                        tracing::info!("Unregistering agent");
                    }
                    SerpeDialect::Heartbeat(_) => {
                        snapshot.append(msg);
                    }
                    SerpeDialect::MissionAccept(_) => {
                        snapshot.append(msg);
                    }
                    SerpeDialect::MissionUpdate(_) => {
                        snapshot.append(msg);
                    }
                    SerpeDialect::MissionFinished(_) => {
                        snapshot.append(msg);
                    }
                    _ => {
                        tracing::warn!("Received message that shouldn't originate from an agent!")
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

pub fn system_communication_send_messages(
    mut query: Query<(&mut SessionConnection, &mut MessageSenderPool)>,
) {
    for (connection, mut sender_pool) in query.iter_mut() {
        for message in sender_pool.iter() {
            let result = connection.outgoing_sender.try_send(message.clone());

            if let Err(err) = result {
                tracing::error!(
                    "Failed to send message to {}: {:?}",
                    connection.system_id,
                    err
                );
            }
        }

        sender_pool.clear();
    }
}
