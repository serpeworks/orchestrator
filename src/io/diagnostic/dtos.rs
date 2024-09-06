use std::fmt;

use serde::Serialize;

use crate::core::{
    communication::ConnectionStatus,
    diagnostic::messages::DiagnosticResponse,
    domain::{OrchestratorState, SessionStatus},
    geo::{Bounds, Coordinates},
};

use super::state::RequestError;

#[derive(Serialize)]
pub struct CoordinatesDto {
    pub lng: f64,
    pub lat: f64,
}

#[derive(Serialize)]
pub struct MissionDto {
    pub active: bool,
    pub target: CoordinatesDto,
    pub waypoints: Vec<CoordinatesDto>,
}

#[derive(Serialize)]
pub struct SessionDetailsDTO {
    pub system_id: u8,
    pub agent_id: u32,
    pub session_id: u32,
    pub session_status: String,
    pub coordinates: CoordinatesDto,
    pub connection_status: String,
    pub mission: Option<MissionDto>,
}

#[derive(Serialize)]
pub struct BoundsDto {
    pub north_west_corner: CoordinatesDto,
    pub size_degrees: f64,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum DTOs {
    ServerInformation {
        state: String,
        uptime: f64,
        tickrate: f64,
        version: String,
    },
    SessionCollection {
        sessions: Vec<SessionDetailsDTO>,
    },
    Environment {
        perimeter: Vec<CoordinatesDto>,
        cells: Vec<BoundsDto>,
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
                tickrate,
            } => DTOs::ServerInformation {
                state: state.to_string(),
                version: version.clone(),
                uptime: *uptime,
                tickrate: *tickrate,
            },
            Self::SessionCollection { sessions } => {
                let sessions_dto = sessions
                    .iter()
                    .map(|session_representation| SessionDetailsDTO {
                        agent_id: session_representation.agent_id,
                        system_id: session_representation.system_id,
                        session_id: session_representation.session_id,
                        session_status: session_representation.session_status.to_string(),
                        coordinates: session_representation.coordinates.to_dto(),
                        connection_status: session_representation.connection_status.to_string(),
                        mission: None, // TODO
                    })
                    .collect();
                DTOs::SessionCollection {
                    sessions: sessions_dto,
                }
            }
            Self::Environment { perimeter, cells } => DTOs::Environment {
                perimeter: perimeter.iter().map(|coord| coord.to_dto()).collect(),
                cells: cells.iter().map(|cell| cell.to_dto()).collect(),
            },
        }
    }
}

impl fmt::Display for OrchestratorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            Self::TooManyRequests => DTOs::Error {
                title: "Too Many Requests".to_string(),
                status: 429,
            },
        }
    }
}

impl Coordinates {
    fn to_dto(self) -> CoordinatesDto {
        CoordinatesDto {
            lng: self.longitude,
            lat: self.latitude,
        }
    }
}

impl Bounds {
    fn to_dto(self) -> BoundsDto {
        BoundsDto {
            north_west_corner: self.north_west_corner.to_dto(),
            size_degrees: self.size_degrees,
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
