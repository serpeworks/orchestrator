use crate::core::{
    communication::SessionConnection,
    domain::{Attitude, GenericResource, SessionInformation},
    geo::EnvironmentResource,
    mission::{Mission, MissionState},
};
use bevy_ecs::system::Query;

use super::messages::{DiagnosticResponse, MissionRepresentation, SessionRepresentation};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn on_server_information(resource: &GenericResource, tickrate: f64) -> DiagnosticResponse {
    let ellapsed = resource.start_time.elapsed();
    DiagnosticResponse::ServerInformation {
        version: VERSION.to_string(),
        state: resource.state,
        uptime: ellapsed.as_secs_f64(),
        tickrate,
    }
}

pub fn on_session_collection(
    sessions: Query<(
        &SessionInformation,
        &SessionConnection,
        &Attitude,
        Option<&Mission>,
    )>,
) -> DiagnosticResponse {
    DiagnosticResponse::SessionCollection {
        sessions: sessions
            .iter()
            .map(
                |(info, connection, attitude, mission_opt)| SessionRepresentation {
                    agent_id: info.agent_id,
                    session_id: info.session_id,
                    session_status: info.session_status,
                    system_id: connection.system_id,
                    connection_status: connection.status,
                    coordinates: attitude.coordinates,
                    mission: mission_opt.map(|mission| MissionRepresentation {
                        active: mission.mission_state == MissionState::ONGOING,
                        target: mission.target,
                        waypoints: mission.waypoints.clone(),
                    }),
                },
            )
            .collect(),
    }
}

pub fn on_environment(environment: &EnvironmentResource) -> DiagnosticResponse {
    DiagnosticResponse::Environment {
        perimeter: environment.perimeter.points.clone(),
        cells: environment.cells.clone(),
    }
}
