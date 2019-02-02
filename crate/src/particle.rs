pub struct Particle {
    pos: [f64; 2],
}

impl Particle {
    pub fn new(pos: [f64; 2]) -> Particle {
        Particle { pos }
    }

    pub fn pos(&self) -> &[f64; 2] {
        &self.pos
    }

    pub fn pos_mut(&mut self) -> &mut [f64; 2] {
        &mut self.pos
    }
}
