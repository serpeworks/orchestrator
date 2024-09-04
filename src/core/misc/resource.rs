use bevy_ecs::system::Resource;

use crate::config::CoreConfiguration;

#[derive(Resource)]
pub struct ConfigurationResource {
    pub config: CoreConfiguration,
}

impl ConfigurationResource {
    pub fn new(config: CoreConfiguration) -> Self {
        Self { config }
    }
}
