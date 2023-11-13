/// Core module for the Drones Server

mod domain;
mod systems;

pub mod configuration;

use tracing::info;
use crate::core::{configuration::Configuration, domain::state::RuntimeState};

pub async fn start_core_task(
    token: tokio_util::sync::CancellationToken,
    configuration: Configuration,
) -> Result<(), ()> {
    let _state = RuntimeState::new();
    info!("Creating State");

    let period = std::time::Duration::from_millis(1000 / configuration.frequency); 
    loop {
        let start = std::time::Instant::now();
        if token.is_cancelled() {
            break
        }



        let ellapsed = start.elapsed();
        sleep(period, ellapsed).await;
    }

    info!("Core Task finishing.");

    return Ok(());
}

async fn sleep(period: std::time::Duration, ellapsed: std::time::Duration) {
    if let Some(remaining) = period.checked_sub(ellapsed) {
        tokio::time::sleep(remaining).await;
    }
}
