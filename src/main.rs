use std::fs::File;

use anyhow::{Ok, Result};
use clap::Parser;
use simulation::Simulation;

use crate::args::Cli;

mod args;
mod constants;
mod day;
mod individual;
mod io;
mod simulation;
mod target;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut simulation = Simulation::new(
        args.particle_count,
        args.transmission_rate,
        args.infectious_period,
        args.mortality_rate,
    );

    let file = File::create(args.xyz_output_path)?;

    let mut status = Vec::new();
    status.push((simulation.date, simulation.infection_status));

    while let Some(mut day) = simulation.next_day() {
        loop {
            io::output_simulation(&file, &day.individuals)?;

            let finished = day.run(args.output_step_count);

            if finished {
                break;
            }
        }

        simulation.update_infection();

        status.push((simulation.date, simulation.infection_status));
    }

    io::output_status(&args.data_output_path, &status)?;

    Ok(())
}
