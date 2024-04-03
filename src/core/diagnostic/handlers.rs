use bevy_ecs::system::Query;

use crate::core::domain::{GenericResource, Session};

use super::messages::DiagnosticResponse;



pub fn on_server_information(
    resource : &GenericResource,
) -> DiagnosticResponse {
    // TODO: Implement this
    let ellapsed = resource.start_time.elapsed();
    DiagnosticResponse::ServerInformation { 
        state: resource.state,
        version: "0.0.1".to_string(),
        uptime: ellapsed.as_secs_f64(),
    } 
}

pub fn on_session_collection(
    sessions : Query<&Session>,
) -> DiagnosticResponse {
    return DiagnosticResponse::SessionCollection {
        sessions: sessions.iter().map(|session| session.session_id).collect(),
    }
}


