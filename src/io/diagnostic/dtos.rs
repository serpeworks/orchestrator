use serde::Serialize;
use crate::core::systems::diagnostic::messages::DiagnosticResponse;

#[derive(Serialize)]
#[serde(untagged)]
pub enum DTOs {
    ServerInformation {
        version: String,
        uptime: f64,
    },
    SessionCollection {
        sessions: Vec<u64>,
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
                sessions: sessions.clone()
            }
        }
    }
}

