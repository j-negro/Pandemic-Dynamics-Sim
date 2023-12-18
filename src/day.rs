use crate::individual::Individual;

pub struct Day<'a> {
    pub individuals: Vec<&'a mut Individual>,
    time: f64,
    pub transmission_rate: f64,
    pub infectious_period: usize,
    pub mortality_rate: f64,
}

impl<'a> Day<'a> {
    pub fn new(
        individuals: Vec<&'a mut Individual>,
        transmission_rate: f64,
        infectious_period: usize,
        mortality_rate: f64,
    ) -> Self {
        Self {
            individuals,
            time: 0.0,
            transmission_rate,
            infectious_period,
            mortality_rate,
        }
    }
}
