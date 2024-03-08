pub mod messages; 

use crate::core::domain::state::RuntimeState;
use self::messages::{DiagnosticMessageReceiver, DiagnosticRequest, DiagnosticResponse};

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
        .map(|(session_id, session)| { (session_id.clone(), session.state.clone()) })
        .collect();

    DiagnosticResponse::SessionCollection { 
        sessions
    }
}

fn process_request(state: &RuntimeState, request: &DiagnosticRequest) -> DiagnosticResponse {
    match request {
        DiagnosticRequest::ServerInformation => on_server_information(state),
        DiagnosticRequest::GetSessionCollection => on_session_collection(state),
    }
}

impl System for DiagnosticSystem {
    fn observe(&mut self, state: &RuntimeState) {
        let _ = self.receiver.try_recv().ok().map(|msg| {
            let response = process_request(&state, &msg.request);
            let _ = msg.send_response(response);
        });
    }

    fn affect(&mut self, _state: &mut RuntimeState) {
        
    }
}


