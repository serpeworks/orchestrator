use std::sync::Arc;

use axum::routing::get;
use axum::{extract::State, Json, Router};
use tower_http::cors::CorsLayer;

use crate::core::systems::diagnostic::messages::DiagnosticRequest;

use super::{dtos::DTOs, state::{AppStateWrapper, AppState}};

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

pub async fn get_environment(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::GetEnvironment;
    let res = state.send_request(request).await.unwrap();
    Json(res.to_dto())
}

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", get(get_root))
        .route("/sessions", get(get_sessions))
        .route("/environment", get(get_environment))
        .layer(CorsLayer::permissive()) //  FIX THIS
        .with_state(Arc::new(state));

    router
}


