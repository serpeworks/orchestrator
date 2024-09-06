use core::panic;
use std::sync::Arc;

use axum::routing::get;
use axum::{extract::State, Json, Router};
use tower_http::cors::CorsLayer;

use crate::config::DiagnosticConfiguration;
use crate::core::diagnostic::messages::DiagnosticRequest;

use super::dtos::ToDTO;
use super::{
    dtos::DTOs,
    state::{AppState, AppStateWrapper},
};

pub async fn get_root(State(state): AppStateWrapper) -> Json<DTOs> {
    let request = DiagnosticRequest::ServerInformation;
    let res = state.send_request(request).await;
    Json(res.to_dto())
}

pub async fn get_sessions(State(state): AppStateWrapper) -> Json<DTOs> {
    let request = DiagnosticRequest::SessionCollection;
    let res = state.send_request(request).await;

    Json(res.to_dto())
}

pub async fn get_environment(State(state): AppStateWrapper) -> Json<DTOs> {
    let request = DiagnosticRequest::Environment;
    let res = state.send_request(request).await;

    Json(res.to_dto())
}

pub fn create_router(state: AppState, config: &DiagnosticConfiguration) -> Router {
    let mut router = Router::new()
        .route("/", get(get_root))
        .route("/sessions", get(get_sessions))
        .route("/environment", get(get_environment))
        .with_state(Arc::new(state));

    if config.use_permissive_cors {
        router = router.layer(CorsLayer::permissive());
    } else {
        panic!("Non-Permissive CORS is not implemented!");
    }

    router
}
