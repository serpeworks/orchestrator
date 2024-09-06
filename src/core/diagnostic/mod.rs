use crate::core::diagnostic::messages::{DiagnosticMessage, DiagnosticResponse};
use bevy_ecs::system::{Query, Res, ResMut, Resource};
use handlers::on_environment;

use self::{
    handlers::{on_server_information, on_session_collection},
    messages::DiagnosticRequest,
};

use super::{
    communication::SessionConnection,
    domain::{Attitude, GenericResource, SessionInformation},
    geo::EnvironmentResource,
    misc::tickrate::TickrateResource,
    mission::Mission,
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
    sessions_query: Query<(
        &SessionInformation,
        &SessionConnection,
        &Attitude,
        Option<&Mission>,
    )>,
    mut diagnostic_resource: ResMut<DiagnosticResource>,
    resource: Res<GenericResource>,
    environment: Res<EnvironmentResource>,
    tickrate: Res<TickrateResource>,
) {
    let _ = diagnostic_resource
        .receiver
        .try_recv()
        .ok()
        .map(|DiagnosticMessage(tx, request)| {
            let response = process_request(
                &request,
                sessions_query,
                &resource,
                &environment,
                tickrate.latest_tickrate,
            );
            let _ = tx.send(response);
        });
}

fn process_request(
    request: &DiagnosticRequest,
    sessions_query: Query<(
        &SessionInformation,
        &SessionConnection,
        &Attitude,
        Option<&Mission>,
    )>,
    generic_resource: &GenericResource,
    environment: &EnvironmentResource,
    tickrate: f64,
) -> DiagnosticResponse {
    match request {
        DiagnosticRequest::ServerInformation => on_server_information(generic_resource, tickrate),
        DiagnosticRequest::SessionCollection => on_session_collection(sessions_query),
        DiagnosticRequest::Environment => on_environment(environment),
    }
}
