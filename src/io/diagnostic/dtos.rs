use serde::Serialize;

use crate::core::{diagnostic::messages::DiagnosticResponse, domain::OrchestratorState};

use super::state::RequestError;

#[derive(Serialize)]
pub struct SessionDetailsDTO {
    pub session_id: u64,
    pub session_state: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum DTOs {
    ServerInformation {
        state: String,
        version: String,
        uptime: f64,
    },
    SessionCollection {
        sessions: Vec<SessionDetailsDTO>,
    },
    Error {
        title: String,
        status: u32,
    }
}

pub trait ToDTO {
    fn to_dto(&self) -> DTOs;
}

impl ToDTO for DiagnosticResponse {
    fn to_dto(&self) -> DTOs {
        match self {
            Self::ServerInformation {state, version, uptime} => DTOs::ServerInformation {
                state: state.to_string(), 
                version: version.clone(),
                uptime: *uptime,
            },
            Self::SessionCollection {sessions} => {
                let sessions = sessions.iter().map(|session_id| {
                    SessionDetailsDTO {
                        session_id: *session_id,
                        session_state: "TODO".to_string(),
                    }
                }).collect();
                DTOs::SessionCollection {
                    sessions,
                }
            }
        }
    }
}

impl OrchestratorState {
    fn to_string(&self) -> String {
        match self {
            Self::Booting => String::from("BOOTING"),
            Self::Running => String::from("RUNNING"),
            Self::Stopping => String::from("STOPPING"),
        }
    }
}

impl ToDTO for RequestError {
    fn to_dto(&self) -> DTOs {
        match self {
            Self::GenericError => DTOs::Error {
                title: "Generic Error".to_string(),
                status: 500,
            },
            Self::_TooManyRequests => DTOs::Error {
                title: "Too Many Requests".to_string(),
                status: 429,
            }
        }
    }
}

impl ToDTO for Result<DiagnosticResponse, RequestError> {
    fn to_dto(&self) -> DTOs {
        match self {
            Ok(response) => response.to_dto(),
            Err(error) => error.to_dto(),
        }
    }
}
