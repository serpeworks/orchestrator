use crate::dialects::SerpeDialect;
use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct MessageSnapshot {
    pub messages: Vec<SerpeDialect>,
}

impl MessageSnapshot {
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn append(&mut self, message: SerpeDialect) {
        self.messages.push(message);
    }

    pub fn iter(&self) -> impl Iterator<Item = &SerpeDialect> {
        self.messages.iter()
    }
}

#[derive(Component, Default)]
pub struct MessageSenderPool {
    pub messages: Vec<SerpeDialect>,
}

impl MessageSenderPool {
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn append(&mut self, message: SerpeDialect) {
        self.messages.push(message);
    }

    pub fn iter(&self) -> impl Iterator<Item = &SerpeDialect> {
        self.messages.iter()
    }
}
