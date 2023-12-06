use super::coords::Coords;

type SessionID = u64;

pub enum SessionState {
    IDLE,
    READY,
    ACTIVE,
}

pub struct Session {
    pub session_id: SessionID,
    pub state: SessionState,
    pub coordinates: Coords,
}


