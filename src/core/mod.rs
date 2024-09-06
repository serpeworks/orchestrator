use std::time::Duration;

use bevy_ecs::prelude::*;
use communication::system_communication_send_messages;
use example_module::system_example_chelas_monitor;
use heartbeat::system_heartbeat;
use kml::Kml;
use misc::{
    clock::{system_clock, Clock},
    system_id_table::SystemIdTable,
    tickrate::system_tickrate,
};
use mission::{system_mission_handler, system_mission_proposal_handler};

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
pub mod example_module;
pub mod geo;
pub mod heartbeat;
pub mod misc;
pub mod mission;

pub async fn start_core_task(
    config: &crate::config::Configuration,
    diagnostic_message_receiver: DiagnosticMessageReceiver,
    communication_incoming_message_receiver: CommsMessageReceiver,
    token: tokio_util::sync::CancellationToken,
    kml: Kml,
) -> Result<(), ()> {
    tracing::info!("Core Loop starting...");

    let mut world = initialize_world(
        config,
        diagnostic_message_receiver,
        communication_incoming_message_receiver,
        kml,
    );

    let mut schedule = create_schedule();

    loop {
        let start = std::time::Instant::now();

        if token.is_cancelled() {
            break;
        }

        schedule.run(&mut world);

        let ellapsed = start.elapsed();
        sleep(config.core.maximum_tickrate, ellapsed).await;
    }

    tracing::info!("Core Task finishing.");

    Ok(())
}

fn initialize_world(
    config: &crate::config::Configuration,
    diagnostic_message_receiver: DiagnosticMessageReceiver,
    communication_incoming_message_receiver: CommsMessageReceiver,
    kml: Kml,
) -> World {
    let mut world = World::default();

    initialize_resources(
        &mut world,
        diagnostic_message_receiver,
        communication_incoming_message_receiver,
        &config.core,
        kml,
    );

    world
}

fn create_schedule() -> Schedule {
    let mut schedule = Schedule::default();

    schedule.add_systems(
        (
            system_clock,
            system_tickrate,
            system_communication_general,
            system_communication_receive_messages,
            system_communication_send_messages,
            system_heartbeat,
            system_mission_proposal_handler,
            system_mission_handler,
            system_diagnostic,
            system_example_chelas_monitor,
        )
            .chain(),
    );

    schedule
}

fn initialize_resources(
    world: &mut World,
    diagnostic_message_receiver: DiagnosticMessageReceiver,
    communication_incoming_message_receiver: CommsMessageReceiver,
    config: &crate::config::CoreConfiguration,
    kml: Kml,
) {
    let start_time = std::time::Instant::now();

    world.insert_resource(Clock::new(&start_time));
    world.insert_resource(SystemIdTable::new());

    world.insert_resource(misc::resource::ConfigurationResource::new(config.clone()));
    world.insert_resource(geo::EnvironmentResource::new(kml, config.cell_size));

    world.insert_resource(domain::GenericResource {
        state: Default::default(),
        start_time,
    });

    world.insert_resource(misc::tickrate::TickrateResource::new(
        config.tickrate_calculation_period_ms,
    ));

    world.insert_resource(diagnostic::DiagnosticResource::new(
        diagnostic_message_receiver,
    ));
    world.insert_resource(communication::CommunicationResource::new(
        communication_incoming_message_receiver,
    ));

    world.insert_resource(mission::MissionHandler::new());
    world.insert_resource(mission::MissionIDCounter::default());

    // example module resource
    world.insert_resource(example_module::MissionRequestTimer::default());
}

async fn sleep(maximum_tickrate: f64, ellapsed: std::time::Duration) {
    let minimum_period = Duration::from_millis(1000 / maximum_tickrate as u64);

    if let Some(remaining) = minimum_period.checked_sub(ellapsed) {
        tokio::time::sleep(remaining).await;
    }
}
