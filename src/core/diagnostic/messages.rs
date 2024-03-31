
use tokio::sync::{oneshot, mpsc};

#[derive(Debug)]
pub enum DiagnosticRequest {
    ServerInformation,
    SessionCollection,
}

pub enum DiagnosticResponse {
    ServerInformation {
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
