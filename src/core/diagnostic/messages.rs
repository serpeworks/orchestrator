
use tokio::sync::{oneshot, mpsc};

use crate::core::domain::OrchestratorState;

#[derive(Debug)]
pub enum DiagnosticRequest {
    ServerInformation,
    SessionCollection,
}

pub enum DiagnosticResponse {
    ServerInformation {
        state: OrchestratorState,
        version: String,
        uptime: f64,
    },
    SessionCollection {
        sessions: Vec<u64>,
    }
}

pub struct DiagnosticMessage (
    pub oneshot::Sender<DiagnosticResponse>, 
    pub DiagnosticRequest,
);

pub type DiagnosticMessageSender = mpsc::Sender<DiagnosticMessage>;
pub type DiagnosticMessageReceiver = mpsc::Receiver<DiagnosticMessage>;

