use tokio::sync::{oneshot, mpsc};

pub enum DiagnosticRequest {
    ServerInformation,
    GetSessionCollection,
}

pub enum DiagnosticResponse {
    ServerInformation {
        version: String,
        uptime: f64,
    },
    SessionCollection {
        sessions: Vec<u64>
    },
}

pub struct DiagnosticMessage (
    pub oneshot::Sender<DiagnosticResponse>, 
    pub DiagnosticRequest,
);

pub type DiagnosticMessageSender = mpsc::Sender<DiagnosticMessage>;
pub type DiagnosticMessageReceiver = mpsc::Receiver<DiagnosticMessage>;

