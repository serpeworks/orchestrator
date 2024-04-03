use std::time::Instant;

use bevy_ecs::{bundle::Bundle, component::Component, system::Resource};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestratorState {
    #[default]
    Booting,
    Running,
    Stopping,
}

#[derive(Default)]
pub enum SessionState {
    #[default]
    Idle,
}

#[derive(Component, Default)]
pub struct Session {
    pub session_id: u64,
    pub session_state: SessionState,
}

#[derive(Bundle, Default)]
pub struct SessionBundle {
    pub session: Session,
}

#[derive(Resource)]
pub struct GenericResource {
    pub state: OrchestratorState,
    pub start_time: Instant,
}

