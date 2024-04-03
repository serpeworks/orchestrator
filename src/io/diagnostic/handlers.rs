use std::sync::Arc;

use axum::routing::get;
use axum::{extract::State, Json, Router};
use tower_http::cors::CorsLayer;

use crate::core::diagnostic::messages::DiagnosticRequest;

use super::dtos::ToDTO;
use super::{dtos::DTOs, state::{AppStateWrapper, AppState}};

pub async fn get_root(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::ServerInformation; 
    let res = state.send_request(request).await;
    Json(res.to_dto())
}

pub async fn get_sessions(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::SessionCollection; 
    let res = state.send_request(request).await;
    Json(res.to_dto())
}

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/", get(get_root))
        .route("/sessions", get(get_sessions))
        .layer(CorsLayer::permissive()) // TODO: correct with a reasonable Cors protection.
        .with_state(Arc::new(state));

    router
}

