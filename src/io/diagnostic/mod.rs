use std::net::SocketAddr;
use std::sync::Arc;

mod dtos;
mod state;

use axum::extract::State;
use axum::{Router, Json};
use axum::routing::get;
use tokio_util::sync::CancellationToken;

use crate::core::systems::diagnostic::messages::{DiagnosticMessageSender, DiagnosticRequest};
use self::dtos::DTOs;

use self::state::{AppState, AppStateWrapper};

pub async fn run_diagnostic_server(
    tx: DiagnosticMessageSender,
    port: u16,
    token: CancellationToken,
) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let state = AppState::new(tx);

    let app = Router::new()
        .route("/", get(get_root))
        .route("/sessions", get(get_sessions))
        .with_state(Arc::new(state));

    let server_task = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled());

    tracing::info!("Diagnostic listening at http://localhost:{}", port);

    let _ = server_task.await;
}

async fn get_root(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::ServerInformation; 
    let res = state.send_request(request).await.unwrap();
    Json(res.to_dto())
}

async fn get_sessions(
    State(state): AppStateWrapper,
) -> Json<DTOs> {
    let request = DiagnosticRequest::GetSessionCollection;
    let res = state.send_request(request).await.unwrap();
    Json(res.to_dto())
}

