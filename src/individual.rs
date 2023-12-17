use crate::{constants::{MIN_PARTICLE_RADIUS, Location}, target::{Target, self}};


pub enum InfectionState {
    Susceptible,
    Infected(u8),
    Recovered,
}

pub struct Individual {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    vx: f64,
    vy: f64,
    pub radius: f64,

    state: InfectionState,
    residence: Location,
    targets: [Target; 3],
    target_idx: usize,
}

impl Individual {
    pub fn new(id: usize, x: f64, y: f64, state: InfectionState, residence: Location) -> Self {
        Self {
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            radius: MIN_PARTICLE_RADIUS,
            state,
            residence,
            targets: target::generate_targets(),
            target_idx: 0,
        }
    }

    
}