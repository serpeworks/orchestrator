use bevy_ecs::prelude::*;
use std::time::{Duration, Instant};

#[derive(Resource)]
pub struct Clock {
    start_time: Instant,
    last_update_time: Instant,
    delta_time: Duration,
}

impl Clock {
    pub fn new(start_time: &Instant) -> Self {
        Clock {
            start_time: *start_time,
            last_update_time: *start_time,
            delta_time: Duration::ZERO,
        }
    }

    pub fn elapsed_time(&self) -> Duration {
        self.last_update_time.duration_since(self.start_time)
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_update_time);
        self.last_update_time = now;
    }

    pub fn now(&self) -> Instant {
        Instant::now()
    }
}

pub fn system_clock(mut clock: ResMut<Clock>) {
    clock.update();
}
