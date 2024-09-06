use std::time::Instant;

use bevy_ecs::{bundle::Bundle, component::Component, system::Resource};

use super::{
    communication::{
        message_pools::{MessageSenderPool, MessageSnapshot},
        SessionConnection,
    },
    geo::Coordinates,
};

pub type AgentID = u32;
pub type SessionID = u32;
pub type SystemID = u8;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestratorState {
    #[default]
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
    pub agent_id: AgentID,
    pub session_id: SessionID,
    pub session_status: SessionStatus,
}

#[derive(Component)]
pub struct Attitude {
    pub coordinates: Coordinates,
}

#[derive(Bundle)]
pub struct SessionBundle {
    pub session: SessionInformation,
    pub attitude: Attitude,
    pub sockets: SessionConnection,
    pub snapshot: MessageSnapshot,
    pub sender_pool: MessageSenderPool,
}

#[derive(Resource)]
pub struct GenericResource {
    pub state: OrchestratorState,
    pub start_time: Instant,
}
