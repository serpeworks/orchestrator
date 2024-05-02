use std::{net::SocketAddr, time::Duration};

mod dtos;
mod handlers;
mod state;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError};
use tokio_util::sync::CancellationToken;
use tower::{
    buffer::BufferLayer,
    limit::{ConcurrencyLimitLayer, RateLimitLayer},
    ServiceBuilder,
};

use crate::{
    core::diagnostic::messages::DiagnosticMessageSender, io::diagnostic::handlers::create_router,
};

use self::state::AppState;

pub async fn run_diagnostic_server(
    tx: DiagnosticMessageSender,
    port: u16,
    token: CancellationToken,
) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let state = AppState::new(tx);

    let app = create_router(state).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                tracing::error!("Error frm Service Builder Layer");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(ConcurrencyLimitLayer::new(256))
            .layer(BufferLayer::new(256))
            .layer(RateLimitLayer::new(256, Duration::from_secs(1))),
    );

    let server_task = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled());

    tracing::info!("Diagnostic listening at http://localhost:{}", port);

    let _ = server_task.await;
}
