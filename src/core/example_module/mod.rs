use bevy_ecs::prelude::*;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

use crate::core::geo::Coordinates;

use super::{
    domain::{Attitude, SessionInformation},
    geo::EnvironmentResource,
    mission::{Mission, MissionHandler},
};

const MISSION_PERIOD_DURATION: Duration = Duration::from_secs(5);

#[derive(Resource)]
pub struct MissionRequestTimer {
    pub last_request: Instant,
}

impl Default for MissionRequestTimer {
    fn default() -> Self {
        Self {
            last_request: Instant::now(),
        }
    }
}

pub fn system_example_chelas_monitor(
    drones_query: Query<(Entity, &Attitude, &SessionInformation), Without<Mission>>,
    environment: Res<EnvironmentResource>,
    mut mission_handler: ResMut<MissionHandler>,
    mut timer: ResMut<MissionRequestTimer>,
) {
    let mut rng = rand::thread_rng();

    if timer.last_request.elapsed() < MISSION_PERIOD_DURATION {
        return;
    }

    for (entity, attitude, session) in drones_query.iter() {
        let current_coordinates = attitude.coordinates;
        let target = environment
            .cells
            .choose(&mut rng)
            .expect("Expected cells in the environment")
            .center();

        tracing::info!(
            "Requesting mission for agent {} to go to {}, {}",
            session.agent_id,
            target.latitude,
            target.longitude,
        );

        // TODO: add waypoints
        let waypoints = generate_waypoints(current_coordinates, target, 16);

        mission_handler.request_mission(entity, target, waypoints);
    }

    timer.last_request = Instant::now();
}

fn generate_waypoints(
    start: Coordinates,
    target: Coordinates,
    num_waypoints: usize,
) -> Vec<Coordinates> {
    let mut waypoints = Vec::with_capacity(num_waypoints);

    for i in 1..=num_waypoints {
        let t = i as f64 / (num_waypoints + 1) as f64;

        // Interpolating latitude and longitude
        let waypoint_latitude = start.latitude + t * (target.latitude - start.latitude);
        let waypoint_longitude = start.longitude + t * (target.longitude - start.longitude);

        waypoints.push(Coordinates {
            latitude: waypoint_latitude,
            longitude: waypoint_longitude,
        });
    }

    waypoints
}
