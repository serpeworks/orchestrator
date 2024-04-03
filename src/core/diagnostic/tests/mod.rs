
use tokio::sync::oneshot;

use super::*;
use crate::core::domain::OrchestratorState;

mod diagnostic_fixture;

#[test]
fn diagnostic_system_server_status_default() {
    // Given
    let mut fixture = diagnostic_fixture::DiagnosticFixture::new();

    // When
    let (tx, mut rx) = oneshot::channel();
    fixture.send_message(DiagnosticMessage(tx, DiagnosticRequest::ServerInformation));
    fixture.run_schedule();

    // Then
    if let DiagnosticResponse::ServerInformation { state, .. } = rx.try_recv().expect("Expect message received") {
        assert_eq!(state, OrchestratorState::Booting)
    } else {
        panic!("Incorrect Response received");
    }
}

#[test]
fn diagnostic_system_session_list_empty_default() {
    // Given
    let mut fixture = diagnostic_fixture::DiagnosticFixture::new();

    // When
    let (tx, mut rx) = oneshot::channel();
    fixture.send_message(DiagnosticMessage(tx, DiagnosticRequest::SessionCollection));
    fixture.run_schedule();

    // Then
    if let DiagnosticResponse::SessionCollection { sessions } = rx.try_recv().expect("Expect message received") {
        assert!(sessions.is_empty())
    } else {
        panic!("Incorrect Response received");
    }

}

#[test]
fn on_server_information_check_state_returned() {
    let mut resource = GenericResource {
        state: OrchestratorState::Booting,
        start_time: std::time::Instant::now(),
    };


    let response = on_server_information(&resource);
    match response {
        DiagnosticResponse::ServerInformation { state, .. } => {
            assert_eq!(state, OrchestratorState::Booting);
        },
        _ => panic!("Expected ServerInformation response"),
    }

    resource.state = OrchestratorState::Running;
    let response = on_server_information(&resource);
    match response {
        DiagnosticResponse::ServerInformation { state, .. } => {
            assert_eq!(state, OrchestratorState::Running);
        },
        _ => panic!("Expected ServerInformation response"),
    }

    resource.state = OrchestratorState::Stopping;
    let response = on_server_information(&resource);
    match response {
        DiagnosticResponse::ServerInformation { state, .. } => {
            assert_eq!(state, OrchestratorState::Stopping);
        },
        _ => panic!("Expected ServerInformation response"),
    }

}

