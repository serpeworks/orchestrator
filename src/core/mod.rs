use bevy_ecs::prelude::*;

use crate::core::{communication::system_communication, diagnostic::system_diagnostic};

use self::diagnostic::messages::DiagnosticMessageReceiver;

pub mod diagnostic;
pub mod communication;
mod domain;

const PERIOD : u64 = 100;

pub async fn start_core_task(
    token: tokio_util::sync::CancellationToken,
    diagnostic_message_receiver: DiagnosticMessageReceiver
) -> Result<(), ()> {
    // Create state and systems
    tracing::info!("Core Loop starting...");

    let mut world = bevy_ecs::world::World::default();
    initialize_resources(&mut world, diagnostic_message_receiver);
    let mut schedule = bevy_ecs::schedule::Schedule::default();

    // Add systems to the schedule
    schedule.add_systems((system_diagnostic, system_communication).chain());

    loop {
        let start = std::time::Instant::now();
        if token.is_cancelled() {
            break
        }

        schedule.run(&mut world);

        let ellapsed = start.elapsed();
        sleep(std::time::Duration::from_millis(PERIOD), ellapsed).await;
    }

    tracing::info!("Core Task finishing.");

    return Ok(());
}

fn initialize_resources(
    world: &mut World,
    diagnostic_message_receiver: DiagnosticMessageReceiver
) {
    if !world.contains_resource::<domain::GenericResource>() {
        world.insert_resource(domain::GenericResource {
            start_time: std::time::Instant::now(),
        });
    }

    if !world.contains_resource::<diagnostic::DiagnosticResource>() {
        world.insert_resource(
            diagnostic::DiagnosticResource::new(diagnostic_message_receiver)
        );
    }
}

async fn sleep(period: std::time::Duration, ellapsed: std::time::Duration) {
    if let Some(remaining) = period.checked_sub(ellapsed) {
        tokio::time::sleep(remaining).await;
    }
}
