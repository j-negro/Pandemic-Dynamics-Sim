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
    #[arg(short, long, default_value_t = 0.5)]
    pub transmission_rate: f64,

    #[arg(short, long, default_value_t = 7)]
    pub infectious_period: usize,

    #[arg(short, long, default_value_t = 0.2)]
    pub mortality_rate: f64,
}
