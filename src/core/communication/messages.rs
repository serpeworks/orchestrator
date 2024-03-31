
pub struct SessionAuthorization {
    pub session_id: u64,
}

// incoming messages
pub enum CommunicationRequests {
    Register {
        drone_id: u64,
    },
    Unregister {
        auth: SessionAuthorization,
    },
    Heartbeat {
        auth: SessionAuthorization,
    },
}

pub enum CommunicationResponses {
    Acknowledge
}

// outgoing messages
pub enum OutgoingMessages {
    Check 
}

pub enum IncomingMessages {
    Acknowledge 
}

