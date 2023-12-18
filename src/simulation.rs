use rand::Rng;

use crate::{
    constants::{distance, MIN_PARTICLE_RADIUS, SIMULATION_LENGHT},
    day::Day,
    individual::{Individual, InfectionState},
};

pub struct Simulation {
    pub individuals: Vec<Individual>,
    pub individual_count: usize,
    pub date: usize,
    pub transmission_rate: f64,
    pub infectious_period: usize,
    pub mortality_rate: f64,
}

impl Simulation {
    pub fn new(
        individual_count: usize,
        transmission_rate: f64,
        infectious_period: usize,
        mortality_rate: f64,
    ) -> Self {
        let mut residences = Vec::with_capacity(individual_count);
        let mut rng = rand::thread_rng();

        // Generate individual residences without overlapping
        for _ in 0..individual_count {
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
        let mut individuals = (0..individual_count)
            .map(|idx| Individual::new(idx, residences[idx]))
            .collect::<Vec<_>>();

        // Infect one individual
        individuals[0].infect(infectious_period);

        Self {
            individuals,
            individual_count,
            date: 0,
            transmission_rate,
            infectious_period,
            mortality_rate,
        }
    }

    pub fn next_day(&mut self) -> Option<Day> {
        todo!()
    }

    pub fn update_infection(&mut self) -> InfectionStatus {
        todo!("Recalculate which particle is infected");

        self.infection_status()
    }

    fn infection_status(&self) -> InfectionStatus {
        let mut status =
            self.individuals
                .iter()
                .fold(InfectionStatus::default(), |mut status, i| {
                    match i.state {
                        InfectionState::Susceptible => status.susceptible += 1,
                        InfectionState::Infected(_) => status.infected += 1,
                        InfectionState::Recovered => status.recovered += 1,
                    }
                    status
                });

        status.dead = self.individual_count - self.individuals.len();

        status
    }
}

#[derive(Debug, Default)]
pub struct InfectionStatus {
    pub susceptible: usize,
    pub infected: usize,
    pub recovered: usize,
    pub dead: usize,
}
