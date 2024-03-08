use std::net::SocketAddr;

mod dtos;
mod state;
mod handlers;

use tokio_util::sync::CancellationToken;


use crate::core::systems::diagnostic::messages::DiagnosticMessageSender;
use crate::io::diagnostic::handlers::create_router;

use self::state::AppState;

pub async fn run_diagnostic_server(
    tx: DiagnosticMessageSender,
    port: u16,
    token: CancellationToken,
) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let state = AppState::new(tx);

    let app = create_router(state);

    let server_task = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(token.cancelled());

    tracing::info!("Diagnostic listening at http://localhost:{}", port);

    let _ = server_task.await;
}
