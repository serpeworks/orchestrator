use axum::{extract::State, Json};

use crate::core::systems::diagnostic::messages::DiagnosticRequest;

use super::{dtos::DTOs, state::AppStateWrapper};

pub async fn get_root(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::ServerInformation; 
    let res = state.send_request(request).await.unwrap();
    Json(res.to_dto())
}

pub async fn get_sessions(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::GetSessionCollection;
    let res = state.send_request(request).await.unwrap();
    Json(res.to_dto())
}

