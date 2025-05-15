use core::time::Duration;

use heapless::Vec;

use crate::units::*;
use crate::NUM_PARTICLES;

pub struct Particle {
    position: Position,
    velocity: Velocity,
    density: Density,
}

impl Particle {
    fn update(&mut self, time_step: Duration, gravity: Acceleration) {
        self.velocity = self.velocity + gravity * time_step;
        self.position = self.position + self.velocity * time_step;
    }

    fn vel(&self) -> Velocity {
        return self.velocity; // copy
    }
}

impl HasPosition for Particle {
    fn get_pos(&self) -> Position {
        return self.position; // copy
    }
}

fn calculate_particle_influence(particles: &mut Vec<Particle, NUM_PARTICLES>) {
    for i in 0..NUM_PARTICLES {}
}
