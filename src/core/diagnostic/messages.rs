use tokio::sync::{mpsc, oneshot};

use crate::core::{
    communication::{ConnectionStatus, SystemID},
    domain::{OrchestratorState, SessionID, SessionStatus},
};

#[derive(Debug)]
pub enum DiagnosticRequest {
    ServerInformation,
    SessionCollection,
}

pub struct SessionRepresentation {
    pub session_id: SessionID,
    pub system_id: SystemID,
    pub session_status: SessionStatus,
    pub connection_status: ConnectionStatus,
}

pub enum DiagnosticResponse {
    ServerInformation {
        state: OrchestratorState,
        version: String,
        uptime: f64,
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
