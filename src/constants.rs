pub const SIMULATION_LENGHT: f64 = 20.0;

pub const MIN_PARTICLE_RADIUS: f64 = 0.1;
pub const MAX_PARTICLE_RADIUS: f64 = 0.37;

pub const MAX_DESIRED_VELOCITY: f64 = 2.0;
pub const BETA: f64 = 0.9;

pub const TIME_STEP: f64 = MIN_PARTICLE_RADIUS / (2.0 * MAX_DESIRED_VELOCITY);
pub const RADIUS_INCREMENT: f64 = MAX_PARTICLE_RADIUS / (0.5 / TIME_STEP);


pub type Location = (f64, f64);

pub const TARGET_RADIUS: f64 = 0.3;
