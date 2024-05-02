use bevy_ecs::prelude::*;

use crate::core::diagnostic::system_diagnostic;

use self::{
    communication::{
        system_communication_general, system_communication_receive_messages, CommsMessageReceiver,
    },
    diagnostic::messages::DiagnosticMessageReceiver,
};

pub mod communication;
pub mod diagnostic;
pub mod domain;

const PERIOD: u64 = 10;

pub async fn start_core_task(
    config: &crate::config::Configuration,
    diagnostic_message_receiver: DiagnosticMessageReceiver,
    communication_incoming_message_receiver: CommsMessageReceiver,
    token: tokio_util::sync::CancellationToken,
) -> Result<(), ()> {
    // Create state and systems
    tracing::info!("Core Loop starting...");

    let mut world = initialize_world(
        config,
        diagnostic_message_receiver,
        communication_incoming_message_receiver,
    );
    let mut schedule = create_schedule();

    loop {
        let start = std::time::Instant::now();
        if token.is_cancelled() {
            break;
        }

        schedule.run(&mut world);

        let ellapsed = start.elapsed();
        sleep(std::time::Duration::from_millis(PERIOD), ellapsed).await;
    }

    tracing::info!("Core Task finishing.");

    Ok(())
}

fn initialize_world(
    config: &crate::config::Configuration,
    diagnostic_message_receiver: DiagnosticMessageReceiver,
    communication_incoming_message_receiver: CommsMessageReceiver,
) -> World {
    let mut world = World::default();

    initialize_resources(
        &mut world,
        diagnostic_message_receiver,
        communication_incoming_message_receiver,
        config,
    );

    world
}

fn create_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(
        (
            system_diagnostic,
            system_communication_general,
            system_communication_receive_messages,
        )
            .chain(),
    );

    schedule
}

fn initialize_resources(
    world: &mut World,
    diagnostic_message_receiver: DiagnosticMessageReceiver,
    communication_incoming_message_receiver: CommsMessageReceiver,
    config: &crate::config::Configuration,
) {
    world.insert_resource(domain::GenericResource {
        version: config.version.clone(),
        state: Default::default(),
        start_time: std::time::Instant::now(),
    });

    world.insert_resource(diagnostic::DiagnosticResource::new(
        diagnostic_message_receiver,
    ));

    world.insert_resource(communication::CommunicationResource::new(
        communication_incoming_message_receiver,
    ));
}

async fn sleep(period: std::time::Duration, ellapsed: std::time::Duration) {
    if let Some(remaining) = period.checked_sub(ellapsed) {
        tokio::time::sleep(remaining).await;
    }
}
