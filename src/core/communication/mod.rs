use bevy_ecs::{component::Component, system::{Res, Resource}};

use crate::dialects::SerpeDialect;

use super::domain::GenericResource;

#[derive(Component)]
struct SessionSocket {
    receiver: tokio::sync::mpsc::Receiver<SerpeDialect>,
    transmitter: tokio::sync::mpsc::Sender<SerpeDialect>    
}

#[derive(Resource)]
pub struct CommunicationResource {
    
}

pub fn system_communication(
    // _communication_resource : ResMut<CommunicationResource>,
    _resource : Res<GenericResource>,
) {
}

pub fn system_communication_entry(
    _resource : Res<GenericResource>,
) {
    
}
