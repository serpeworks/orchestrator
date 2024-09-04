use bevy_ecs::system::{Res, ResMut, Resource};
use std::time::{Duration, Instant};

use super::clock::Clock;

#[derive(Resource)]
pub struct TickrateResource {
    pub latest_tickrate: f64,
    period_start: Instant,
    ticks: u32,
    tickrate_calculation_period: Duration,
}

impl TickrateResource {
    pub fn new(tickrate_calculation_period_ms: u64) -> Self {
        Self {
            latest_tickrate: 0.0,
            period_start: Instant::now(),
            ticks: 0,
            tickrate_calculation_period: Duration::from_millis(tickrate_calculation_period_ms),
        }
    }

    pub fn tickrate_step(&mut self, now: Instant) {
        let elapsed = self.period_start.elapsed();

        if elapsed >= self.tickrate_calculation_period {
            self.latest_tickrate = self.ticks as f64 / elapsed.as_secs_f64();

            self.period_start = now;
            self.ticks = 0;
        }

        self.ticks += 1;
    }
}

pub fn system_tickrate(mut tickrate: ResMut<TickrateResource>, clock: Res<Clock>) {
    tickrate.tickrate_step(clock.now());
}
