use bevy_ecs::system::{Query, Res, ResMut, Resource};
use crate::core::diagnostic::messages::{DiagnosticMessage, DiagnosticResponse};

use self::messages::DiagnosticRequest;

use super::domain::{GenericResource, Session};
pub mod messages;
#[derive(Resource)]
pub struct DiagnosticResource {
    receiver: messages::DiagnosticMessageReceiver,
}

impl DiagnosticResource {
    pub fn new(receiver: messages::DiagnosticMessageReceiver) -> Self {
        Self {
            receiver,
        }
    }
}

pub fn system_diagnostic(
    mut diagnostic_resource : ResMut<DiagnosticResource>,
    _resource : Res<GenericResource>,
    sessions : Query<&Session>,
) {

    let _ = diagnostic_resource.receiver.try_recv().ok().map(|DiagnosticMessage(tx, request)| {
        let response = process_request(&request, sessions, &_resource);
        let _ = tx.send(response);
    });
}


fn process_request(
    request: &DiagnosticRequest,
    sessions : Query<&Session>,
    generic_resource: &GenericResource,
) -> DiagnosticResponse {
    match request {
        DiagnosticRequest::ServerInformation => on_server_information(&generic_resource),
        DiagnosticRequest::SessionCollection => on_session_collection(sessions),
    }
}

fn on_server_information(
    resource : &GenericResource,
) -> DiagnosticResponse {
    // TODO: Implement this
    let ellapsed = resource.start_time.elapsed();
    DiagnosticResponse::ServerInformation { 
        version: "0.0.1".to_string(),
        uptime: ellapsed.as_secs_f64(),
    } 
}

fn on_session_collection(
    sessions : Query<&Session>,
) -> DiagnosticResponse {
    return DiagnosticResponse::SessionCollection {
        sessions: sessions.iter().map(|session| session.session_id).collect(),
    }
}
