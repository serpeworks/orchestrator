pub mod messages; 

use crate::core::domain::state::RuntimeState;
use self::messages::{DiagnosticMessageReceiver, DiagnosticRequest, DiagnosticResponse, DiagnosticMessage};

use super::System;

pub struct DiagnosticSystem {
    receiver: DiagnosticMessageReceiver,
}

impl DiagnosticSystem {
    pub fn new(rx: DiagnosticMessageReceiver) -> Self {
        Self {
            receiver: rx, 
        }
    }
}

fn on_server_information(state: &RuntimeState) -> DiagnosticResponse {
    let uptime = state.get_elapsed_time();
    DiagnosticResponse::ServerInformation { 
        version: "0.0.1".to_string(),
        uptime,
    } 
}

fn on_session_collection(state: &RuntimeState) -> DiagnosticResponse {
    let sessions = state.sessions.iter()
        .map(|item| { return *item.0 })
        .collect();

    DiagnosticResponse::SessionCollection { 
        sessions
    }
}

fn on_environment(_state: &RuntimeState) -> DiagnosticResponse {
    todo!()
}

fn process_request(state: &RuntimeState, request: &DiagnosticRequest) -> DiagnosticResponse {
    match request {
        DiagnosticRequest::ServerInformation => on_server_information(state),
        DiagnosticRequest::GetSessionCollection => on_session_collection(state),
        DiagnosticRequest::GetEnvironment => on_environment(state),
    }
}

impl System for DiagnosticSystem {
    fn observe(&mut self, state: &RuntimeState) {
        let _ = self.receiver.try_recv().ok().map(|DiagnosticMessage(tx, request)| {
            let response = process_request(&state, &request);
            let _ = tx.send(response);
        });
    }

    fn affect(&mut self, _state: &mut RuntimeState) {
        
    }
}


