use std::sync::Arc;
use axum::extract::State;
use tokio::sync::oneshot;

use crate::core::diagnostic::messages::{DiagnosticMessage, DiagnosticMessageSender, DiagnosticRequest, DiagnosticResponse};

#[derive(Debug)]
pub enum RequestError {
    GenericError,
    _TooManyRequests,
}

pub type AppStateWrapper = State<Arc<AppState>>;

pub struct AppState {
    tx: DiagnosticMessageSender,
}


impl AppState {
    pub fn new(
        tx: DiagnosticMessageSender,
    ) -> Self {
        Self {
            tx, 
        }
    }

    pub async fn send_request(
        &self,
        request: DiagnosticRequest,
    ) -> Result<DiagnosticResponse, RequestError> {
        let (tx, rx) = oneshot::channel(); 
        let msg = DiagnosticMessage(tx, request);
        match self.tx.try_send(msg) {
            Ok(_) => {},
            Err(_) => return Err(RequestError::GenericError),
        }

        let response = rx.await.unwrap();
        Ok(response)
    }
}
