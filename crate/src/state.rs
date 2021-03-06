use crate::particle::Particle;
use rand::distributions::{Distribution, Normal};
use rand::prelude::ThreadRng;

pub struct State {
    particles: Vec<Particle>,
    rng: ThreadRng,
    max_bounds: [f64; 2],
    normal_distribution: Normal,
    speed: f64,
}

impl State {
    pub fn new(width: f64, height: f64) -> State {
        let mut particles = Vec::new();
        let rng = rand::thread_rng();

        let speed = 3.0; // standard deviation**2 of the normal distribution
        let normal_distribution = Normal::new(0.0, speed);

        for _ in 1..100 {
            particles.push(Particle::new([width / 2.0, height / 2.0]));
        }

        State {
            particles,
            rng,
            max_bounds: [width, height],
            normal_distribution: normal_distribution,
            speed,
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

    pub fn update_num_particles(&mut self, new_size: usize) {
        let diff = new_size as isize - self.particles.len() as isize;
        if diff < 0 {
            self.particles.drain(new_size..);
        } else {
            for _ in 0..diff.abs() {
                self.particles.push(Particle::new([
                    self.max_bounds[0] / 2.0,
                    self.max_bounds[1] / 2.0,
                ]));
            }
        }
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn update_speed(&mut self, speed: f64) {
        self.normal_distribution = Normal::new(0.0, speed);
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }
}
