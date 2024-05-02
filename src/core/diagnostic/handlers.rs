use bevy_ecs::system::Query;

use crate::core::{
    communication::SessionConnection,
    domain::{GenericResource, SessionInformation},
};

use super::messages::{DiagnosticResponse, SessionRepresentation};

pub fn on_server_information(resource: &GenericResource) -> DiagnosticResponse {
    // TODO: Implement this
    let ellapsed = resource.start_time.elapsed();
    DiagnosticResponse::ServerInformation {
        state: resource.state,
        version: resource.version.clone(),
        uptime: ellapsed.as_secs_f64(),
    }
}

pub fn on_session_collection(
    sessions: Query<(&SessionInformation, &SessionConnection)>,
) -> DiagnosticResponse {
    return DiagnosticResponse::SessionCollection {
        sessions: sessions
            .iter()
            .map(|(info, connection)| SessionRepresentation {
                session_id: info.session_id,
                session_status: info.session_status,
                system_id: connection.system_id,
                connection_status: connection.status,
            })
            .collect(),
    };
}
