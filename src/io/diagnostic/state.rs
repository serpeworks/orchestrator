use std::sync::Arc;
use axum::extract::State;
use tokio::sync::oneshot;

use crate::core::systems::diagnostic::messages::{DiagnosticMessageSender, DiagnosticRequest, DiagnosticResponse, DiagnosticMessage};

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
    ) -> Result<DiagnosticResponse, ()> {
        let (tx, rx) = oneshot::channel(); 
        let msg = DiagnosticMessage(tx, request);
        let _ = self.tx.send(msg).await;
        
        let response = rx.await.unwrap();
        Ok(response)
    }
}
