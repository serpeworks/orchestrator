use std::time::Instant;

use bevy_ecs::{bundle::Bundle, component::Component, system::Resource};

use super::communication::SessionConnection;

pub type AgentID = u32;
pub type SessionID = u32;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestratorState {
    #[default]
    Booting,
    Running,
    Stopping,
}

#[derive(Clone, Copy, Default)]
pub enum SessionStatus {
    #[default]
    Idle,
}

#[derive(Component, Default)]
pub struct SessionInformation {
    pub session_id: SessionID,
    pub agent_id: AgentID,
    pub session_status: SessionStatus,
}

#[derive(Bundle)]
pub struct SessionBundle {
    pub session: SessionInformation,
    pub sockets: SessionConnection,
}

#[derive(Resource)]
pub struct GenericResource {
    pub version: String,
    pub state: OrchestratorState,
    pub start_time: Instant,
}
