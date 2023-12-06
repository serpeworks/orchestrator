pub mod diagnostic;

use super::domain::state::RuntimeState;

pub trait System {
    fn observe(&mut self, state: &RuntimeState);
    fn affect(&mut self, state: &mut RuntimeState);
}


