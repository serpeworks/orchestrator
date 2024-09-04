use bevy_ecs::{schedule::Schedule, world::World};
use tokio::sync::mpsc;

use super::{
    messages::DiagnosticMessage, system_diagnostic, DiagnosticResource, GenericResource,
    TickrateResource,
};

pub struct DiagnosticFixture {
    world: World,
    schedule: Schedule,
    pub tx: mpsc::Sender<DiagnosticMessage>,
}

impl DiagnosticFixture {
    pub fn new() -> Self {
        let mut world = World::default();

        let (tx, receiver) = mpsc::channel(1);
        world.insert_resource(DiagnosticResource { receiver });
        world.insert_resource(TickrateResource::new(200));

        world.insert_resource(GenericResource {
            state: Default::default(),
            start_time: std::time::Instant::now(),
        });

        let mut schedule = Schedule::default();
        schedule.add_systems(system_diagnostic);

        DiagnosticFixture {
            world,
            schedule,
            tx,
        }
    }

    pub fn send_message(&mut self, message: DiagnosticMessage) {
        self.tx
            .blocking_send(message)
            .expect("Expected sending message to be fine.");
    }

    pub fn run_schedule(&mut self) {
        self.schedule.run(&mut self.world)
    }
}
