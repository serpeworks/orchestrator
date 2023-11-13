use std::net::SocketAddr;
use std::sync::Arc;
mod state;

use axum::extract::State;
use axum::{Router, Json};
use axum::routing::get;
use tokio_util::sync::CancellationToken;
use serde::Serialize;

use self::state::{AppState, AppStateWrapper};

#[derive(Serialize)]
struct ServerInformationDTO {
    version: String,
    uptime: f64,
}

pub async fn run_diagnostic_server(
    port: u16,
    token: CancellationToken
) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let state = AppState::new();

    let app = Router::new()
        .route("/", get(root))
        .with_state(Arc::new(state));

    let _ = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled())
        .await;
}

async fn root(
) -> Json<ServerInformationDTO> {
    // TODO: make this be returned from the server.
    let dto = ServerInformationDTO {
        version: "0.0.1".to_string(),
        uptime: 0.0
    };

    Json(dto)
}
