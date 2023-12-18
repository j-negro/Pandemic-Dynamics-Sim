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
    pub infection_status: InfectionStatus,
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

        let infection_status = InfectionStatus::new(&individuals, individual_count);

        Self {
            individuals,
            individual_count,
            date: 0,
            transmission_rate,
            infectious_period,
            mortality_rate,
            infection_status,
        }
    }

    pub fn next_day(&mut self) -> Option<Day> {
        (self.infection_status.infected > 0).then(|| {
            self.date += 1;
            Day::new(
                self.individuals.iter_mut().collect(),
                self.transmission_rate,
            )
        })
    }

    pub fn update_infection(&mut self) {
        let mut rng = rand::thread_rng();

        self.individuals.retain_mut(|i| {
            match i.state {
                InfectionState::Susceptible => {
                    if i.to_infect {
                        i.infect(self.infectious_period);
                    }
                }
                InfectionState::Infected(0) => {
                    let p = rng.gen_range(0f64..1f64);

                    if p < self.mortality_rate {
                        i.state = InfectionState::Recovered;
                    } else {
                        return false;
                    }
                }
                InfectionState::Infected(n) => i.state = InfectionState::Infected(n - 1),
                InfectionState::Recovered => (),
            };

            i.reset_to_residence();

            i.recreate_random_target();

            true
        });

        self.infection_status = InfectionStatus::new(&self.individuals, self.individual_count);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InfectionStatus {
    pub susceptible: usize,
    pub infected: usize,
    pub recovered: usize,
    pub dead: usize,
}

impl InfectionStatus {
    pub fn new(individuals: &[Individual], initial_count: usize) -> Self {
        let mut status = individuals
            .iter()
            .fold(InfectionStatus::default(), |mut status, i| {
                match i.state {
                    InfectionState::Susceptible => status.susceptible += 1,
                    InfectionState::Infected(_) => status.infected += 1,
                    InfectionState::Recovered => status.recovered += 1,
                }
                status
            });

        status.dead = initial_count - individuals.len();

        status
    }
}
