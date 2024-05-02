use std::fmt;

use serde::Serialize;

use crate::core::{
    communication::ConnectionStatus,
    diagnostic::messages::DiagnosticResponse,
    domain::{OrchestratorState, SessionStatus},
};

use super::state::RequestError;

#[derive(Serialize)]
pub struct SessionDetailsDTO {
    pub system_id: u8,
    pub session_id: u32,
    pub session_status: String,
    pub connection_status: String,
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
    },
}

pub trait ToDTO {
    fn to_dto(&self) -> DTOs;
}

impl ToDTO for DiagnosticResponse {
    fn to_dto(&self) -> DTOs {
        match self {
            Self::ServerInformation {
                state,
                version,
                uptime,
            } => DTOs::ServerInformation {
                state: state.to_string(),
                version: version.clone(),
                uptime: *uptime,
            },
            Self::SessionCollection { sessions } => {
                let sessions = sessions
                    .iter()
                    .map(|session_representation| SessionDetailsDTO {
                        system_id: session_representation.system_id,
                        session_id: session_representation.session_id,
                        session_status: session_representation.session_status.to_string(),
                        connection_status: session_representation.connection_status.to_string(),
                    })
                    .collect();
                DTOs::SessionCollection { sessions }
            }
        }
    }
}

impl fmt::Display for OrchestratorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Booting => write!(f, "BOOTING"),
            Self::Running => write!(f, "RUNNING"),
            Self::Stopping => write!(f, "STOPPING"),
        }
    }
}

impl fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "IDLE"),
        }
    }
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "CONNECTED"),
            Self::Disconnected => write!(f, "DISCONNECTED"),
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
            },
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
