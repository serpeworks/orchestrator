use bevy_ecs::prelude::*;
use std::collections::HashSet;

use crate::core::domain::SystemID;

#[derive(Resource)]
pub struct SystemIdTable {
    used_ids: HashSet<SystemID>,
}

impl Default for SystemIdTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemIdTable {
    pub fn new() -> Self {
        SystemIdTable {
            used_ids: HashSet::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.used_ids.len()
    }

    pub fn allocate(&mut self) -> Option<SystemID> {
        for id in 1..=255 {
            if !self.used_ids.contains(&id) {
                self.used_ids.insert(id);
                return Some(id);
            }
        }
        None
    }

    pub fn release(&mut self, id: SystemID) {
        self.used_ids.remove(&id);
    }
}
