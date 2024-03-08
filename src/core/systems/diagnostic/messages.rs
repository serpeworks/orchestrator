use tokio::sync::{oneshot, mpsc};

use crate::core::domain::session::SessionState;

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
        sessions: Vec<(u64, SessionState)>
    },
}

pub struct DiagnosticMessage {
    sender: oneshot::Sender<DiagnosticResponse>, 
    pub request: DiagnosticRequest,
}

impl DiagnosticMessage {
    pub fn new(sender: oneshot::Sender<DiagnosticResponse>, request: DiagnosticRequest) -> Self {
        Self {
            sender,
            request,
        }
    }

    pub fn send_response(self, response: DiagnosticResponse) {
        let _ = self.sender.send(response);
    }
}

pub type DiagnosticMessageSender = mpsc::Sender<DiagnosticMessage>;
pub type DiagnosticMessageReceiver = mpsc::Receiver<DiagnosticMessage>;

