use bevy_ecs::prelude::*;

use crate::dialects::{
    serpe_dialect::messages::{MissionAcceptAck, MissionFinishedAck, MissionRequest},
    SerpeDialect,
};

use super::{
    communication::message_pools::{MessageSenderPool, MessageSnapshot},
    domain::SessionInformation,
    geo::Coordinates,
};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum MissionState {
    PROPOSED,
    ONGOING,
}

pub type MissionID = u64;

#[derive(Clone, Debug, Component)]
pub struct Mission {
    pub mission_id: MissionID,
    pub mission_state: MissionState,
    pub waypoints: Vec<Coordinates>,
    pub target: Coordinates,
}

#[derive(Clone, Component)]
pub struct MissionProposal {
    pub entity: Entity,
    pub target: Coordinates,
    pub waypoints: Vec<Coordinates>,
}

#[derive(Resource)]
pub struct MissionHandler {
    mission_proposals: Vec<MissionProposal>,
}

#[derive(Resource)]
pub struct MissionIDCounter {
    count: MissionID,
}

impl MissionIDCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Default for MissionIDCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl MissionHandler {
    pub fn new() -> Self {
        MissionHandler {
            mission_proposals: Vec::new(),
        }
    }

    pub fn request_mission(
        &mut self,
        entity: Entity,
        target: Coordinates,
        waypoints: Vec<Coordinates>,
    ) {
        if waypoints.len() > 32 {
            tracing::warn!("Tried to create a mission with too many waypoints");
            return;
        }

        let mission_proposal = MissionProposal {
            entity,
            target,
            waypoints,
        };

        self.mission_proposals.push(mission_proposal);
    }
}

pub fn system_mission_proposal_handler(
    mut drones_query: Query<(&mut MessageSenderPool, Option<&Mission>)>,
    mut mission_handler: ResMut<MissionHandler>,
    mut mission_id_counter: ResMut<MissionIDCounter>,
    mut commands: Commands,
) {
    for MissionProposal {
        entity,
        target,
        waypoints,
    } in mission_handler.mission_proposals.iter()
    {
        match drones_query.get_mut(*entity) {
            Ok((mut message_sender, mission_opt)) => {
                if mission_opt.is_some() {
                    tracing::info!("Tried to assign mission to an agent that already has a mission")
                }

                message_sender.append(SerpeDialect::MissionRequest(MissionRequest {
                    target_latitude: target.latitude as f32,
                    target_longitude: target.longitude as f32,
                    waypoint_count: 0,
                    waypoint_latitudes: [0.0; 32],
                    waypoint_longitudes: [0.0; 32],
                }));

                commands.entity(*entity).insert(Mission {
                    mission_id: mission_id_counter.count,
                    mission_state: MissionState::PROPOSED,
                    waypoints: waypoints.clone(),
                    target: *target,
                });

                mission_id_counter.count += 1;
            }
            Err(_) => {}
        }
    }
    mission_handler.mission_proposals.clear();
}

pub fn system_mission_handler(
    mut drones_query: Query<(
        Entity,
        &SessionInformation,
        &MessageSnapshot,
        &mut MessageSenderPool,
        &mut Mission,
    )>,
    mut commands: Commands,
) {
    for (entity, session, snapshot, mut sender_pool, mut mission) in drones_query.iter_mut() {
        match mission.mission_state {
            MissionState::PROPOSED => {
                for msg in snapshot.iter() {
                    if let SerpeDialect::MissionAccept(_) = msg {
                        mission.mission_state = MissionState::ONGOING;

                        sender_pool.append(SerpeDialect::MissionAcceptAck(MissionAcceptAck {}));

                        tracing::info!(
                            "Mission for entity {:?} accepted, sending MissionAcceptAck.",
                            entity
                        );

                        break;
                    }
                }
            }
            MissionState::ONGOING => {
                for msg in snapshot.iter() {
                    match msg {
                        SerpeDialect::MissionUpdate(_) => {
                            // ignore current mission updates
                        }
                        SerpeDialect::MissionFinished(_) => {
                            tracing::info!(
                                "Agent {} has finished mission {}.",
                                session.agent_id,
                                mission.mission_id
                            );

                            sender_pool
                                .append(SerpeDialect::MissionFinishedAck(MissionFinishedAck {}));

                            commands.entity(entity).remove::<Mission>();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Default for MissionHandler {
    fn default() -> Self {
        Self::new()
    }
}
