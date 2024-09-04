use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, Query, Res, ResMut, Resource},
};
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    config::CommunicationConfiguration,
    core::domain::{SessionBundle, SessionInformation, SessionStatus},
    dialects::{self, serpe_dialect, SerpeDialect},
};

use super::{
    domain::{AgentID, GenericResource, OrchestratorState, SystemID},
    misc::{resource::ConfigurationResource, system_id_table::SystemIdTable},
};

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
    generic_resource: Res<GenericResource>,
    config_resource: Res<ConfigurationResource>,
    mut resource: ResMut<CommunicationResource>,
    mut id_table: ResMut<SystemIdTable>,
    mut commands: Commands,
) {
    if generic_resource.state == OrchestratorState::Booting {
        return;
    }

    while let Ok(message) = resource.receiver.try_recv() {
        match message {
            CommsMessage::Register {
                agent_id,
                receiver,
                sender,
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
                if let None = assigned_system_id_opt {
                    continue;
                }

                let system_id = assigned_system_id_opt.unwrap();
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
                    sockets: SessionConnection {
                        system_id,
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
    mut id_table: ResMut<SystemIdTable>,
) {
    for (entity, _, mut connection) in query.iter_mut() {
        loop {
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
