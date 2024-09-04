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
    config::DiagnosticConfiguration, core::diagnostic::messages::DiagnosticMessageSender,
    io::diagnostic::handlers::create_router,
};

use self::state::AppState;

pub async fn run_diagnostic_server(
    tx: DiagnosticMessageSender,
    config: DiagnosticConfiguration,
    token: CancellationToken,
) {
    if !config.enabled {
        return;
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let state = AppState::new(tx);

    let app = create_router(state, &config).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                tracing::error!("Error frm Service Builder Layer");
                ( StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(ConcurrencyLimitLayer::new(config.concurrent_requests))
            .layer(BufferLayer::new(config.buffer_size))
            .layer(RateLimitLayer::new(
                config.rate_limit,
                Duration::from_secs(1),
            )),
    );

    let server_task = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled());

    tracing::info!(
        "Diagnostic listening at http://{}:{}",
        config.host,
        config.port
    );

    let _ = server_task.await;
}
