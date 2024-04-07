use bevy_ecs::{
    component::Component,
    system::{Res, ResMut, Resource},
};

use crate::dialects::SerpeDialect;

use super::domain::GenericResource;

pub struct CommunicationPipes {
    pub receiver: tokio::sync::mpsc::Receiver<SerpeDialect>,
    pub transmitter: tokio::sync::mpsc::Sender<SerpeDialect>,
}

#[derive(Component)]
struct SessionSocket {
    _pipes: CommunicationPipes,
}

#[derive(Resource)]
pub struct CommunicationResource {
    _receiver: tokio::sync::mpsc::Receiver<CommunicationPipes>,
}

impl CommunicationResource {
    pub fn new(_receiver: tokio::sync::mpsc::Receiver<CommunicationPipes>) -> Self {
        CommunicationResource { _receiver }
    }
}

pub fn system_communication(
    _communication_resource: ResMut<CommunicationResource>,
    _resource: Res<GenericResource>,
) {
}
