use std::collections::HashMap;

use super::session::Session;


pub struct RuntimeState {
    sessions: HashMap<u64, Session> 
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new()
        }
    }
    
    pub fn register_session(&mut self) -> Result<u64, ()> {
        
        Ok(0) 
    }

    pub fn obtain_session(&self, session_id: &u64) -> Option<&Session> {
        self.sessions.get(session_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_a_session_on_empty_state() {
        // Arrange
        // A newly created state
        let mut state = RuntimeState::new();

        // Act
        // Register a new session on the state
        let result = state.register_session();

        // Assert
        // The result was problem free and returned a new id
        assert!(result.is_ok());
        let session_id = result.expect("Expected the result to be ok!");
        
        // The result ID can be looked up on the state
        let session = state.obtain_session(&session_id);
    }
}
