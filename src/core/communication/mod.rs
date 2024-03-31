use bevy_ecs::system::{Commands, Resource};

pub mod messages;

#[derive(Resource)]
pub struct CommunicationResource {

}

pub fn system_communication(
    // _communication_resource : ResMut<CommunicationResource>,
    mut _commands: Commands,
) {
    
}

