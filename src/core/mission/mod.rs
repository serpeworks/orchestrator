use bevy_ecs::prelude::*;

use super::geo::Coordinates;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum MissionState {
    PROPOSED,
    ONGOING,
    FINISHED,
}

#[derive(Clone, Debug, Component)]
pub struct Mission {
    pub mission_state: MissionState,
    pub waypoints: Vec<Coordinates>,
    pub target: Coordinates,
}

pub fn system_mission() {}
