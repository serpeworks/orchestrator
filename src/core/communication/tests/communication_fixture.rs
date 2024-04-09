use bevy_ecs::{schedule::Schedule, world::World};

use crate::core::communication::{
    system_communication, CommunicationResource, SerpeDialectReceiver, SerpeDialectSender,
};

pub struct CommunicationFixture {
    pub world: World,
    pub schedule: Schedule,
    pub incoming_sender: SerpeDialectSender,
    pub outgoing_receiver: SerpeDialectReceiver,
}

const COMMUNICATION_CHANNEL_SIZE: usize = 256;

impl CommunicationFixture {
    pub fn new() -> Self {
        let mut world = World::default();

        let (outgoing_sender, outgoing_receiver) =
            tokio::sync::mpsc::channel(COMMUNICATION_CHANNEL_SIZE);
        let (incoming_sender, incoming_receiver) =
            tokio::sync::mpsc::channel(COMMUNICATION_CHANNEL_SIZE);
        world.insert_resource(CommunicationResource::new(
            incoming_receiver,
            outgoing_sender,
        ));

        let mut schedule = Schedule::default();
        schedule.add_systems(system_communication);

        CommunicationFixture {
            world,
            schedule,
            incoming_sender,
            outgoing_receiver,
        }
    }

    pub fn run_schedule(&mut self) {
        self.schedule.run(&mut self.world)
    }

    pub fn connect_session(&mut self) {}
}
