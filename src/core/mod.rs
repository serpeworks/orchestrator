/// Core module for the Drones Server.

mod state; 

use tracing::info;

use crate::core::state::RuntimeState;

pub async fn start_core_task(
    token: tokio_util::sync::CancellationToken
) -> Result<(), ()> {
    let _state = RuntimeState::new();
    info!("Creating State");

    loop {
        if token.is_cancelled() {
            break
        }


        tokio::task::yield_now().await;
    }

    info!("Core Task finishing.");

    return Ok(());
}

