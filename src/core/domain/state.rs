use std::{collections::HashMap, time::Instant};

use super::session::{Session, SessionState};


pub struct RuntimeState {
    pub sessions: HashMap<u64, Session>,

    current_session_id: u64, // Temporary way of generation sessions

    start_time: Instant,
}

impl RuntimeState {
    pub fn new(
        start_time: Instant
    ) -> Self {
        Self {
            sessions: HashMap::new(),
            current_session_id: 0,
            start_time,
        }
    }

    pub fn generate_session_id(&mut self) -> u64 {
        self.current_session_id += 1;
        self.current_session_id
    }
    
    pub fn register_session(&mut self) -> Result<u64, ()> {
        // Create a session_id 
        let session_id = self.generate_session_id();
        
        // Create Session
        let session = Session {
            session_id,
            state: SessionState::IDLE,
        };

        // Save Session
        self.sessions.insert(session_id, session);

        Ok(session_id) 
    }

    pub fn obtain_session(&self, session_id: &u64) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    pub fn get_elapsed_time(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registering_new_session() {
        // Arrange
        let mut state = RuntimeState::new(Instant::now());

        // Act
        let result = state.register_session();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn registering_session_then_retrieve_it() {
        // Arrange
        let mut state = RuntimeState::new(Instant::now());
        let result = state.register_session();
        let session_id = result.expect("Expected the result to be ok!");

        // Act
        let session = state.obtain_session(&session_id);

        // Assert
        assert!(session.is_some(), "Retrieving session was not possible!");
    }

    #[test]
    fn registering_session_then_retrieving_it_has_same_id() {
        // Arrange
        let mut state = RuntimeState::new(Instant::now());
        let result = state.register_session();
        let session_id = result.expect("Expected the result to be ok!");

        // Act
        let session = state.obtain_session(&session_id).expect("Session should be present!");
        
        // Assert
        assert_eq!(session_id, session.session_id);
    }
}
