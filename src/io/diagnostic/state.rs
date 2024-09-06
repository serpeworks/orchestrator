use axum::extract::State;
use std::sync::Arc;
use tokio::sync::oneshot;

use crate::core::diagnostic::messages::{
    DiagnosticMessage, DiagnosticMessageSender, DiagnosticRequest, DiagnosticResponse,
};

#[derive(Debug)]
pub enum RequestError {
    GenericError,
    TooManyRequests,
}

pub type AppStateWrapper = State<Arc<AppState>>;

pub struct AppState {
    tx: DiagnosticMessageSender,
}

impl AppState {
    pub fn new(tx: DiagnosticMessageSender) -> Self {
        Self { tx }
    }

    pub async fn send_request(
        &self,
        request: DiagnosticRequest,
    ) -> Result<DiagnosticResponse, RequestError> {
        let (tx, rx) = oneshot::channel();
        let msg = DiagnosticMessage(tx, request);
        match self.tx.try_send(msg) {
            Ok(_) => {}
            Err(send_error) => match send_error {
                tokio::sync::mpsc::error::TrySendError::Full(_) => {
                    return Err(RequestError::TooManyRequests)
                }
                tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                    return Err(RequestError::GenericError);
                }
            },
        }

        match rx.await {
            Ok(response) => Ok(response),
            Err(_) => Err(RequestError::GenericError),
        }
    }
}
