use bevy_ecs::{component::Component, system::{Res, Resource}};

use super::domain::GenericResource;

#[derive(Component)]
struct SessionSocket {
    
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
