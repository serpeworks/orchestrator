use serde::Serialize;

use crate::core::diagnostic::messages::DiagnosticResponse;

#[derive(Serialize)]
pub struct SessionDetailsDTO {
    pub session_id: u64,
    pub session_state: String,
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

impl DiagnosticResponse {
    pub fn to_dto(&self) -> DTOs {
        match self {
            Self::ServerInformation {version, uptime} => DTOs::ServerInformation {
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

