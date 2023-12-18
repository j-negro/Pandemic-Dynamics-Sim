use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Pandemic Dynamics Sim", author, version, about)]
pub struct Cli {
    // Output parameters
    #[arg(short, long, default_value_t = 1)]
    pub output_step_count: usize,

    #[arg(short, long, default_value_t = String::from("./simulation.xyz"))]
    pub xyz_output_path: String,

    #[arg(short, long, default_value_t = String::from("./particles_time.txt"))]
    pub data_output_path: String,

    // Simulation input variables
    #[arg(short, long, default_value_t = 0.1, value_parser = validate_rate)]
    pub transmission_rate: f64,

    #[arg(short, long, default_value_t = 7)]
    pub infectious_period: usize,

    #[arg(short, long, default_value_t = 0.05, value_parser = validate_rate)]
    pub mortality_rate: f64,

    #[arg(short, long, default_value_t = 200)]
    pub particle_count: usize,
}

fn validate_rate(rate: &str) -> Result<f64, String> {
    let rate = rate
        .parse::<f64>()
        .map_err(|_| "Rate specified is not a number.")?;

    if !(0.0..=1.0).contains(&rate) {
        return Err("Please specify a number between 0 and 1.".to_owned());
    }
    Ok(rate)
}
