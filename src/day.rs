use crate::{constants::TIME_STEP, individual::Individual};

pub struct Day<'a> {
    pub individuals: Vec<&'a mut Individual>,
    time: f64,
    pub transmission_rate: f64,
}

impl<'a> Day<'a> {
    pub fn new(individuals: Vec<&'a mut Individual>, transmission_rate: f64) -> Self {
        Self {
            individuals,
            time: 0.0,
            transmission_rate,
        }
    }

    pub fn run(&mut self, steps: usize) -> bool {
        for _ in 0..steps {
            self.time += TIME_STEP;

            let mut collisions = Vec::with_capacity(self.individuals.len());

            // NOTE: Calculate wall collisions
            for individual in &self.individuals {
                collisions.push(individual.check_wall_collisions());
            }

            // NOTE: Calculate particle collisions
            for i in 0..self.individuals.len() {
                for j in i + 1..self.individuals.len() {
                    if self.individuals[i].is_colliding(self.individuals[j]) {
                        let particle_coords = self.individuals[i].get_coordinates();
                        let colliding_coords = self.individuals[j].get_coordinates();

                        collisions[i].push(colliding_coords);
                        collisions[j].push(particle_coords);

                        if self.individuals[j].is_infected() && !self.individuals[i].is_infected() {
                            self.individuals[i].to_infect += 1;
                        }

                        if self.individuals[i].is_infected() && !self.individuals[j].is_infected() {
                            self.individuals[j].to_infect += 1;
                        }
                    }
                }
            }

            // NOTE: Update velocity and radius
            for (idx, individual) in self.individuals.iter_mut().enumerate() {
                if collisions[idx].is_empty() {
                    individual.update_desired();
                } else {
                    individual.update_escape(&collisions[idx]);
                }
            }

            // NOTE: Step individuals forward
            let mut to_remove = Vec::new();
            for individual in &mut self.individuals {
                individual.step();

                let reached_home = individual.update_target();

                if reached_home {
                    to_remove.push(individual.id);
                }
            }

            // NOTE: Remove particles that reached its home
            self.individuals.retain(|p| !to_remove.contains(&p.id));
        }

        self.individuals.is_empty()
    }
}
