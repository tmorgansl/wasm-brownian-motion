use crate::particle::Particle;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct State {
    particles: Vec<Particle>,
    rng: ThreadRng,
    max_bounds: [f64; 2],
}

impl State {
    pub fn new(width: f64, height: f64) -> State {
        let mut particles = Vec::new();
        let rng = rand::thread_rng();

        for _ in 1..10000 {
            particles.push(Particle::new([width / 2.0, height / 2.0]));
        }

        State { particles, rng, max_bounds: [width, height] }
    }

    pub fn tick(&mut self) {
        for particle in self.particles.iter_mut() {
            for i in 0..2 {
                let new_position = particle.pos()[i] + self.rng.gen_range(-10.0, 10.0);

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
