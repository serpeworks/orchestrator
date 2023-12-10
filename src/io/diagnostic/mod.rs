use std::net::SocketAddr;
use std::sync::Arc;

mod dtos;
mod state;
mod handlers;

use axum::extract::State;
use axum::{Router, Json};
use axum::routing::get;
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;


use crate::core::systems::diagnostic::messages::{DiagnosticMessageSender, DiagnosticRequest};
use crate::io::diagnostic::handlers::{get_root, get_sessions};
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
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state));

    let server_task = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled());

    tracing::info!("Diagnostic listening at http://localhost:{}", port);

    let _ = server_task.await;
}
