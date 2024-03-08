type SessionID = u64;

#[derive(Clone, Copy, Debug)]
pub enum SessionState {
    IDLE,
}

pub struct Session {
    pub session_id: SessionID,
    pub state: SessionState,
}


