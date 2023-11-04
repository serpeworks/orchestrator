/// Core module for the Drones Server.

mod state; 

pub mod configuration;

use tracing::info;

use crate::core::state::RuntimeState;

use self::configuration::Configuration;

#[tracing::instrument]
pub async fn start_core_task(
    token: tokio_util::sync::CancellationToken,
    configuration: Configuration,
) -> Result<(), ()> {
    let _state = RuntimeState::new();
    info!("Creating State");

    let period = 1000 / configuration.frequency; 
    loop {
        let start = std::time::Instant::now();
        if token.is_cancelled() {
            break
        }





        let ellapsed = start.elapsed();
        if let Some(remaining) = std::time::Duration::from_millis(period).checked_sub(ellapsed) {
            tokio::time::sleep(remaining).await;
        }
    }

    info!("Core Task finishing.");

    return Ok(());
}

