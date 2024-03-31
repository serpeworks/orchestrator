use std::time::Instant;

use bevy_ecs::{bundle::Bundle, component::Component, system::Resource};

#[allow(dead_code)]
#[derive(Default)]
enum OrchestratorState {
    #[default]
    Booting,
    Running,
    ShuttingDown,
}

#[derive(Default)]
pub enum SessionState {
    #[default]
    _Idle,
}

#[derive(Component)]
pub struct Session {
    pub session_id: u64,
    pub session_state: SessionState,
}

#[derive(Bundle)]
pub struct SessionBundle {
    pub session: Session,
}

#[derive(Resource)]
pub struct GenericResource {
    pub start_time: Instant,
}

