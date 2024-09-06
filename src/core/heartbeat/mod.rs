use super::{
    communication::message_pools::{MessageSenderPool, MessageSnapshot},
    domain::Attitude,
};
use crate::dialects::{serpe_dialect::messages::HeartbeatAck, SerpeDialect};
use bevy_ecs::prelude::*;

pub fn system_heartbeat(
    mut query: Query<(&mut Attitude, &MessageSnapshot, &mut MessageSenderPool)>,
) {
    for (mut attitude, snapshot, mut sender_pool) in query.iter_mut() {
        for message_enum in snapshot.iter() {
            if let SerpeDialect::Heartbeat(heartbeat_message) = message_enum {
                attitude.coordinates.longitude = heartbeat_message.longitude as f64;
                attitude.coordinates.latitude = heartbeat_message.latitude as f64;

                let ack_message = SerpeDialect::HeartbeatAck(HeartbeatAck {});

                sender_pool.append(ack_message);
            }
        }
    }
}
