use bevy_ecs::system::Query;

use crate::core::{
    communication::SessionConnection,
    domain::{GenericResource, SessionInformation},
};

use super::messages::{DiagnosticResponse, SessionRepresentation};

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
    sessions: Query<(&SessionInformation, &SessionConnection)>,
) -> DiagnosticResponse {
    return DiagnosticResponse::SessionCollection {
        sessions: sessions
            .iter()
            .map(|(info, connection)| SessionRepresentation {
                agent_id: info.agent_id,
                session_id: info.session_id,
                session_status: info.session_status,
                system_id: connection.system_id,
                connection_status: connection.status,
                mission: None
            })
            .collect(),
    };
}
