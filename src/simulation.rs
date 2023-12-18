use rand::Rng;

use crate::{
    constants::{distance, MIN_PARTICLE_RADIUS, SIMULATION_LENGHT},
    individual::{Individual, InfectionState},
};

pub struct Simulation {
    pub individuals: Vec<Individual>,
    pub date: usize,
    pub transmission_rate: f64,
    pub infectious_period: usize,
    pub mortality_rate: f64,
}

impl Simulation {
    pub fn new(
        particle_count: usize,
        transmission_rate: f64,
        infectious_period: usize,
        mortality_rate: f64,
    ) -> Self {
        let mut residences = Vec::with_capacity(particle_count);
        let mut rng = rand::thread_rng();

        // Generate individual residences without overlapping
        for _ in 0..particle_count {
            loop {
                let x =
                    rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));
                let y =
                    rng.gen_range(MIN_PARTICLE_RADIUS..=(SIMULATION_LENGHT - MIN_PARTICLE_RADIUS));

                let residence = (x, y);

                if residences.iter().any(|other_residence| {
                    distance(residence, *other_residence) < MIN_PARTICLE_RADIUS
                }) {
                    continue;
                }

                residences.push(residence);
                break;
            }
        }

        // Generate all individuals
        let mut individuals = (0..particle_count)
            .map(|idx| Individual::new(idx, InfectionState::Susceptible, residences[0]))
            .collect::<Vec<_>>();
        // Infect one individual
        individuals[0].infect(infectious_period);

        Self {
            individuals,
            date: 0,
            transmission_rate,
            infectious_period,
            mortality_rate,
        }
    }
}
