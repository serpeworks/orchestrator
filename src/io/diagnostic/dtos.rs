use serde::Serialize;
use crate::core::{domain::session::SessionState, systems::diagnostic::messages::DiagnosticResponse};

#[derive(Serialize)]
pub struct SessionDetailsDTO {
    pub session_id: u64,
    pub drone_state: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum DTOs {
    ServerInformation {
        version: String,
        uptime: f64,
    },
    SessionCollection {
        sessions: Vec<SessionDetailsDTO>,
    },
}

trait ToDTO<T>{
    fn to_dto(&self) -> T;
}

impl ToDTO<String> for SessionState {
    fn to_dto(&self) -> String {
        match self {
            SessionState::IDLE => "IDLE".to_string()
        }
    }
}

impl DiagnosticResponse {
    pub fn to_dto(&self) -> DTOs {
        match self {
            Self::ServerInformation {version, uptime} => DTOs::ServerInformation {
                version: version.clone(),
                uptime: *uptime,
            },
            Self::SessionCollection { sessions } => DTOs::SessionCollection {
                sessions: sessions.iter()
                    .map(|(session_id, session_state)| 
                        SessionDetailsDTO {
                            session_id: session_id.clone(),
                            drone_state: session_state.to_dto()
                        }
                    ).collect()
            },
        }
    }
}

