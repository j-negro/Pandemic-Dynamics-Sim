use std::fs::File;

use anyhow::{Ok, Result};
use clap::Parser;
use simulation::Simulation;

use crate::args::Cli;

mod args;
mod constants;
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
    io::output_simulation(&file, &simulation.individuals)?;

    // let mut removed_times = Vec::new();

    // let mut i = 0;
    // loop {
    //     removed_times.append(&mut simulation.run(args.output_step_count));

    //     io::output_simulation(&file, &simulation.particles, &simulation.target)?;

    //     if simulation.particles.is_empty() {
    //         break;
    //     }

    //     if let Some(max) = args.max_steps {
    //         if i > max {
    //             break;
    //         }
    //     }

    //     i += 1;
    // }

    // io::output_times(&args.data_output_path, &removed_times)?;

    Ok(())
}
