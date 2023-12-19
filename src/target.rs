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
        let coords = individual.get_coordinates();
        let dx = self.location.0 - coords.0;
        let dy = self.location.1 - coords.1;

        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        distance <= self.radius + individual.get_radius()
    }
}

pub fn generate_targets(last: Location) -> [Target; 3] {
    let mut targets = (0..2)
        .map(|_| Target::new(TARGET_RADIUS))
        .collect::<Vec<_>>();

    targets.push(Target {
        location: last,
        radius: TARGET_RADIUS,
    });

    targets.try_into().unwrap()
}
