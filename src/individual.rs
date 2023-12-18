use crate::{
    constants::{
        distance, Location, BETA, MAX_DESIRED_VELOCITY, MAX_PARTICLE_RADIUS, MIN_PARTICLE_RADIUS,
        RADIUS_INCREMENT, SIMULATION_LENGHT, TIME_STEP,
    },
    target::{self, Target},
};

pub enum InfectionState {
    Susceptible,
    Infected(usize),
    Recovered,
}

pub struct Individual {
    pub id: usize,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    // Infection Information
    pub state: InfectionState,
    pub to_infect: bool,
    residence: Location,
    targets: [Target; 3],
    target_idx: usize,
}

impl Individual {
    pub fn new(id: usize, residence: Location) -> Self {
        Self {
            id,
            x: residence.0,
            y: residence.1,
            vx: 0.0,
            vy: 0.0,
            radius: MIN_PARTICLE_RADIUS,
            state: InfectionState::Susceptible,
            to_infect: false,
            residence,
            targets: target::generate_targets(),
            target_idx: 0,
        }
    }

    // Infection Functions

    pub fn infect(&mut self, infected_period: usize) {
        self.state = InfectionState::Infected(infected_period)
    }

    pub fn reset_to_residence(&mut self) {
        self.set_coordinates(self.residence);
    }

    // CPM Functions

    fn distance(&self, other: Location) -> f64 {
        // TODO: Is this even useful?
        distance(self.get_coordinates(), other)
    }

    pub fn is_colliding(&self, other: &Individual) -> bool {
        self.distance(other.get_coordinates()) <= self.radius + other.radius
    }

    pub fn check_wall_collisions(&self) -> Vec<Location> {
        let mut wall_collisions = Vec::new();

        if self.x - self.radius <= 0.0 {
            wall_collisions.push((0.0, self.y));
        } else if self.x + self.radius >= SIMULATION_LENGHT {
            wall_collisions.push((SIMULATION_LENGHT, self.y));
        }

        if self.y - self.radius <= 0.0 {
            wall_collisions.push((self.x, 0.0));
        } else if self.y + self.radius >= SIMULATION_LENGHT {
            wall_collisions.push((self.x, SIMULATION_LENGHT));
        }

        wall_collisions
    }

    pub fn step(&mut self) {
        self.x += self.vx * TIME_STEP;
        self.y += self.vy * TIME_STEP;
    }

    fn calculate_desired_velocity(radius: f64) -> f64 {
        MAX_DESIRED_VELOCITY
            * ((radius - MIN_PARTICLE_RADIUS) / (MAX_PARTICLE_RADIUS - MIN_PARTICLE_RADIUS))
                .powf(BETA)
    }

    pub fn update_escape(&mut self, collision_points: &[(f64, f64)]) {
        self.radius = MIN_PARTICLE_RADIUS;

        let mut collision_vector = (0.0, 0.0);

        for (x, y) in collision_points {
            let diff = (self.x - x, self.y - y);
            let norm = (diff.0.powi(2) + diff.1.powi(2)).sqrt();

            collision_vector.0 += diff.0 / norm;
            collision_vector.1 += diff.1 / norm;
        }

        let norm = (collision_vector.0.powi(2) + collision_vector.1.powi(2)).sqrt();

        self.vx = MAX_DESIRED_VELOCITY * collision_vector.0 / norm;
        self.vy = MAX_DESIRED_VELOCITY * collision_vector.1 / norm;
    }

    pub fn update_desired(&mut self) {
        if self.radius < MAX_PARTICLE_RADIUS {
            self.radius += RADIUS_INCREMENT;
        }

        let desired_velocity = Self::calculate_desired_velocity(self.radius);

        let target_location = self.targets[self.target_idx].location;
        let target_direction = (target_location.0 - self.x, target_location.1 - self.y);
        let target_norm = (target_direction.0.powi(2) + target_direction.1.powi(2)).sqrt();

        self.vx = desired_velocity * target_direction.0 / target_norm;
        self.vy = desired_velocity * target_direction.1 / target_norm;
    }

    pub fn get_velocities(&self) -> (f64, f64) {
        (self.vx, self.vy)
    }

    pub fn get_coordinates(&self) -> Location {
        (self.x, self.y)
    }

    pub fn set_coordinates(&mut self, coords: Location) {
        self.x = coords.0;
        self.y = coords.1;
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}
