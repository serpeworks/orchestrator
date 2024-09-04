use crate::core::diagnostic::messages::{DiagnosticMessage, DiagnosticResponse};
use bevy_ecs::system::{Query, Res, ResMut, Resource};

use self::{
    handlers::{on_server_information, on_session_collection},
    messages::DiagnosticRequest,
};

use super::{
    communication::SessionConnection,
    domain::{GenericResource, SessionInformation}, misc::tickrate::TickrateResource,
};

mod handlers;
pub mod messages;

#[cfg(test)]
mod tests;

#[derive(Resource)]
pub struct DiagnosticResource {
    receiver: messages::DiagnosticMessageReceiver,
}

impl DiagnosticResource {
    pub fn new(receiver: messages::DiagnosticMessageReceiver) -> Self {
        Self { receiver }
    }
}

pub fn system_diagnostic(
    mut diagnostic_resource: ResMut<DiagnosticResource>,
    resource: Res<GenericResource>,
    tickrate: Res<TickrateResource>,
    sessions: Query<(&SessionInformation, &SessionConnection)>,
) {
    let _ = diagnostic_resource
        .receiver
        .try_recv()
        .ok()
        .map(|DiagnosticMessage(tx, request)| {
            let response = process_request(&request, sessions, &resource, tickrate.latest_tickrate);
            let _ = tx.send(response);
        });
}

fn process_request(
    request: &DiagnosticRequest,
    sessions: Query<(&SessionInformation, &SessionConnection)>,
    generic_resource: &GenericResource,
    tickrate: f64,
) -> DiagnosticResponse {
    match request {
        DiagnosticRequest::ServerInformation => on_server_information(generic_resource, tickrate),
        DiagnosticRequest::SessionCollection => on_session_collection(sessions),
    }
}
