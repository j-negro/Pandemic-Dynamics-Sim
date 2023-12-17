use rand::Rng;

use crate::{
    constants::{Location, MIN_PARTICLE_RADIUS, SIMULATION_LENGHT, TARGET_RADIUS},
    individual::Individual,
};

#[derive(Debug, Clone)]
pub struct Target {
    pub location: Location,
    pub radius: f64,
}

impl Target {
    pub fn new(radius: f64) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));
        let y = rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));

        Self {
            location: (x, y),
            radius,
        }
    }

    pub fn in_target(&self, individual: &Individual) -> bool {
        let dx = self.location.0 - individual.x;
        let dy = self.location.1 - individual.y;

        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        distance <= self.radius + individual.radius
    }
}

pub fn generate_targets() -> [Target; 3] {
    let targets = (0..3)
        .map(|_| Target::new(TARGET_RADIUS))
        .collect::<Vec<_>>();

    targets.try_into().unwrap()
}
