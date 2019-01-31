use crate::particle::Particle;
use rand::distributions::{Distribution, Normal};
use rand::prelude::ThreadRng;
use std::time::{Duration, Instant};

pub struct State {
    particles: Vec<Particle>,
    rng: ThreadRng,
    max_bounds: [f64; 2],
    normal_distribution: Normal,
}

impl State {
    pub fn new(width: f64, height: f64) -> State {
        let mut particles = Vec::new();
        let rng = rand::thread_rng();
        let normal_distribution = Normal::new(0.0, 3.0);

        for _ in 1..1000 {
            particles.push(Particle::new([width / 2.0, height / 2.0]));
        }

        State {
            particles,
            rng,
            max_bounds: [width, height],
            normal_distribution: normal_distribution,
        }
    }

    pub fn tick(&mut self) {
        for particle in self.particles.iter_mut() {
            for i in 0..2 {
                let new_position =
                    particle.pos()[i] + self.normal_distribution.sample(&mut self.rng);

                if new_position > 0.0 && new_position < self.max_bounds[i] {
                    particle.pos_mut()[i] = new_position;
                }
            }
        }
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }
}
