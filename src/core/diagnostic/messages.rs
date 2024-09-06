use tokio::sync::{mpsc, oneshot};

use crate::core::{
    communication::ConnectionStatus,
    domain::{AgentID, OrchestratorState, SessionID, SessionStatus, SystemID},
    geo::{Bounds, Coordinates},
    mission::MissionID,
};

#[derive(Debug)]
pub enum DiagnosticRequest {
    ServerInformation,
    Environment,
    SessionCollection,
}

#[derive(Clone)]
pub struct MissionRepresentation {
    pub mission_id: MissionID,
    pub active: bool,
    pub target: Coordinates,
    pub waypoints: Vec<Coordinates>,
}

pub struct SessionRepresentation {
    pub agent_id: AgentID,
    pub session_id: SessionID,
    pub system_id: SystemID,
    pub session_status: SessionStatus,
    pub connection_status: ConnectionStatus,
    pub coordinates: Coordinates,
    pub mission: Option<MissionRepresentation>,
}

pub enum DiagnosticResponse {
    ServerInformation {
        state: OrchestratorState,
        version: String,
        uptime: f64,
        tickrate: f64,
    },
    Environment {
        perimeter: Vec<Coordinates>,
        cells: Vec<Bounds>,
    },
    SessionCollection {
        sessions: Vec<SessionRepresentation>,
    },
}

pub struct DiagnosticMessage(
    pub oneshot::Sender<DiagnosticResponse>,
    pub DiagnosticRequest,
);

pub type DiagnosticMessageSender = mpsc::Sender<DiagnosticMessage>;
pub type DiagnosticMessageReceiver = mpsc::Receiver<DiagnosticMessage>;
