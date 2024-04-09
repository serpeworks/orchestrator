use bevy_ecs::{component::Component, system::Resource};

use crate::dialects::SerpeDialect;

#[cfg(test)]
mod tests;

pub type SerpeDialectReceiver = tokio::sync::mpsc::Receiver<SerpeDialect>;
pub type SerpeDialectSender = tokio::sync::mpsc::Sender<SerpeDialect>;

#[derive(Component)]
struct SessionSocket {
    _incoming_receiver: SerpeDialectReceiver,
    _outgoing_sender: SerpeDialectSender,
}

#[derive(Resource)]
pub struct CommunicationResource {
    incoming_receiver: SerpeDialectReceiver,
    outgoing_sender: SerpeDialectSender,
}

impl CommunicationResource {
    pub fn new(
        incoming_receiver: SerpeDialectReceiver,
        outgoing_sender: SerpeDialectSender,
    ) -> Self {
        CommunicationResource {
            incoming_receiver,
            outgoing_sender,
        }
    }
}

pub fn system_communication() {
    // Handle new registrations
}
