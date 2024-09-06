use bevy_ecs::{schedule::Schedule, world::World};
use kml::Kml;
use tokio::sync::mpsc;

use super::{
    messages::DiagnosticMessage, system_diagnostic, DiagnosticResource, EnvironmentResource,
    GenericResource, TickrateResource,
};

pub struct DiagnosticFixture {
    world: World,
    schedule: Schedule,
    pub tx: mpsc::Sender<DiagnosticMessage>,
}

impl DiagnosticFixture {
    pub fn new() -> Self {
        let mut world = World::default();

        let kml_str = r#"
        <Polygon>
            <outerBoundaryIs>
                <LinearRing>
                    <coordinates>
                        -9.119521257471535,38.75780573956479,0 -9.118893141837919,38.75344875753808,0 -9.117138252193829,38.75247033533404,0 -9.113938825947761,38.75147866920116,0 -9.113871211836262,38.75409600431954,0 -9.11321695545595,38.75556572036042,0 -9.106866340672514,38.75396188437206,0 -9.104882234553243,38.75631601937288,0 -9.111926359727981,38.75853953864151,0 -9.11151884548644,38.76028790438408,0 -9.1127099163848,38.76230468029339,0 -9.116694751039628,38.7636031970526,0 -9.120464349060107,38.76230211054566,0 -9.119521257471535,38.75780573956479,0 
                    </coordinates>
                </LinearRing>
            </outerBoundaryIs>
        </Polygon>
        "#;

        let kml: Kml = kml_str.parse().unwrap();

        let (tx, receiver) = mpsc::channel(1);
        world.insert_resource(DiagnosticResource { receiver });
        world.insert_resource(TickrateResource::new(200));
        world.insert_resource(EnvironmentResource::new(kml, 0.005));

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

impl Default for DiagnosticFixture {
    fn default() -> Self {
        Self::new()
    }
}
