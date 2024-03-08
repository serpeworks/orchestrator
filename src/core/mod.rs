/// Core module for the Drones Server

pub mod domain;
pub mod systems;

use tracing::info;
use crate::core::{domain::state::RuntimeState, systems::{diagnostic::DiagnosticSystem, System}};

use self::systems::diagnostic::messages::DiagnosticMessageReceiver;

const FREQUENCY : u64 = 200;

pub async fn start_core_task(
    rx: DiagnosticMessageReceiver,
    token: tokio_util::sync::CancellationToken,
) -> Result<(), ()> {
    // Create state and systems
    let mut state = RuntimeState::new(std::time::Instant::now());
    let mut systems : Vec<Box<dyn System>> = vec![
        Box::new(DiagnosticSystem::new(rx))
    ];
   
    let period = std::time::Duration::from_millis(1000 / FREQUENCY); 

    tracing::info!("Core Loop starting...");
    loop {
        let start = std::time::Instant::now();
        if token.is_cancelled() {
            break
        }
        
        // First do observation step
        systems.iter_mut().for_each(|sys| sys.observe(&state));

        // Perform affect step
        systems.iter_mut().for_each(|sys| sys.affect(&mut state));
        

        sleep(period, start.elapsed()).await;
    }

    info!("Core Task finishing.");

    return Ok(());
}

async fn sleep(period: std::time::Duration, ellapsed: std::time::Duration) {
    if let Some(remaining) = period.checked_sub(ellapsed) {
        tokio::time::sleep(remaining).await;
    }
}
