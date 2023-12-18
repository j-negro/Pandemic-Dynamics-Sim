use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{constants::SIMULATION_LENGHT, individual::Individual, Result};

const CORNERS: [(f64, f64); 4] = [
    (0.0, 0.0),
    (SIMULATION_LENGHT, 0.0),
    (0.0, SIMULATION_LENGHT),
    (SIMULATION_LENGHT, SIMULATION_LENGHT),
];

pub fn output_simulation(file: &File, particles: &Vec<Individual>) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let particle_count = particles.len() + CORNERS.len();
    writeln!(writer, "{particle_count}")?;
    writeln!(
        writer,
        "Properties=pos:R:2:velo:R:2:radius:R:1:color:R:3 pbc=\"F F\"",
    )?;

    // NOTE: Write the particles
    for particle in particles {
        let coordinates = particle.get_coordinates();
        let velocities = particle.get_velocities();

        writeln!(
            writer,
            "{:.12} {:.12} {:.12} {:.12} {:.4} 1 1 1",
            coordinates.0,
            coordinates.1,
            velocities.0,
            velocities.1,
            particle.get_radius(),
        )?;
    }

    // NOTE: Write the corners
    CORNERS
        .into_iter()
        .try_for_each(|c| writeln!(writer, "{:.12} {:.12} 0 0 0.05 1 1 1", c.0, c.1))?;

    Ok(())
}

pub fn output_times(path: &str, times: &Vec<(usize, f64)>) -> Result<()> {
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);

    for (time, count) in times {
        writeln!(writer, "{time} {count}")?;
    }

    Ok(())
}
