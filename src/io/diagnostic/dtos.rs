use serde::Serialize;
use crate::core::systems::diagnostic::messages::DiagnosticResponse;

#[derive(Serialize)]
pub struct SessionDetailsDTO {
    pub session_id: u64,
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

impl DiagnosticResponse {
    pub fn to_dto(&self) -> DTOs {
        match self {
            Self::ServerInformation {version, uptime} => DTOs::ServerInformation {
                version: version.clone(),
                uptime: *uptime,
            },
            Self::SessionCollection { sessions } => DTOs::SessionCollection {
                sessions: vec![]
            },
        }
    }
}

