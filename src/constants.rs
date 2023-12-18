pub const SIMULATION_LENGHT: f64 = 200.0;

pub const MIN_PARTICLE_RADIUS: f64 = 0.1;
pub const MAX_PARTICLE_RADIUS: f64 = 0.37;

pub const MAX_DESIRED_VELOCITY: f64 = 2.0;
pub const BETA: f64 = 0.9;

pub const TIME_STEP: f64 = MIN_PARTICLE_RADIUS / (2.0 * MAX_DESIRED_VELOCITY);
pub const RADIUS_INCREMENT: f64 = MAX_PARTICLE_RADIUS / (0.5 / TIME_STEP);

pub type Location = (f64, f64);

pub const TARGET_RADIUS: f64 = 0.3;

pub fn distance(this: Location, other: Location) -> f64 {
    let dx = this.0 - other.0;
    let dy = this.1 - other.1;

    (dx.powi(2) + dy.powi(2)).sqrt()
}
