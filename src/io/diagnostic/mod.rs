use std::net::SocketAddr;

use axum::{Router, Json};
use axum::routing::get;
use tokio_util::sync::CancellationToken;
use serde::Serialize;

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

    let app = Router::new()
        .route("/", get(root));

    let _ = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled())
        .await;
}

async fn root() -> Json<ServerInformationDTO> {
    let dto = ServerInformationDTO {
        version: "0.0.1".to_string(),
        uptime: 0.0
    };

    Json(dto)
}
