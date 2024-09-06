use bevy_ecs::{schedule::Schedule, world::World};

use crate::core::{
    communication::{
        system_communication_general, system_communication_receive_messages, CommsMessage,
        CommsMessageSender, CommunicationResource, SerpeDialectReceiver, SerpeDialectSender,
    },
    geo::Coordinates,
    misc::{resource::ConfigurationResource, system_id_table::SystemIdTable},
};

pub struct CommunicationFixture {
    pub world: World,
    pub schedule: Schedule,
    pub sender: CommsMessageSender,
}

const COMMUNICATION_CHANNEL_SIZE: usize = 256;

impl CommunicationFixture {
    pub fn new() -> Self {
        let mut world = World::default();

        let (sender, receiver) = tokio::sync::mpsc::channel(COMMUNICATION_CHANNEL_SIZE);
        world.insert_resource(CommunicationResource::new(receiver));
        world.insert_resource(SystemIdTable::new());
        world.insert_resource(ConfigurationResource::new(
            crate::config::CoreConfiguration {
                max_number_of_drones: 100,
                maximum_tickrate: 100.0,
                tickrate_calculation_period_ms: 10,
                perimeter_filepath: "".to_string(),
                cell_size: 0.0005,
            },
        ));
        
        let mut schedule = Schedule::default();
        schedule.add_systems((
            system_communication_general,
            system_communication_receive_messages,
        ));

        CommunicationFixture {
            world,
            schedule,
            sender,
        }
    }

    pub fn run_schedule(&mut self) {
        self.schedule.run(&mut self.world)
    }

    pub fn connect_session(&mut self, agent_id: u32) -> (SerpeDialectSender, SerpeDialectReceiver) {
        let (incoming_sender, incoming_receiver) =
            tokio::sync::mpsc::channel(COMMUNICATION_CHANNEL_SIZE);
        let (outgoing_sender, outgoing_receiver) =
            tokio::sync::mpsc::channel(COMMUNICATION_CHANNEL_SIZE);

        self.sender
            .try_send(CommsMessage::Register {
                agent_id,
                receiver: incoming_receiver,
                sender: outgoing_sender,
                coordinates: Coordinates::default(),
            })
            .unwrap();

        (incoming_sender, outgoing_receiver)
    }
}
