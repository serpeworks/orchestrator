/// Core module for the Drones Server.

mod state; 

use tracing::info;

use crate::core::state::RuntimeState;

#[tracing::instrument]
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
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    info!("Core Task finishing.");

    return Ok(());
}

