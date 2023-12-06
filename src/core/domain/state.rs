use std::{collections::HashMap, time::Instant};

use super::{session::{Session, SessionState}, coords::Coords};


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
            coordinates: Coords::new(0.0, 0.0, 0.0),
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
    fn register_a_session_on_empty_state() {
        // Arrange
        // A newly created state
        let mut state = RuntimeState::new(Instant::now());

        // Act
        // Register a new session on the state
        let result = state.register_session();

        // Assert
        // The result was problem free and returned a new id
        assert!(result.is_ok());
        let session_id = result.expect("Expected the result to be ok!");
        
        // TODO: The result ID can be looked up on the state
        let _session = state.obtain_session(&session_id);
    }
}
