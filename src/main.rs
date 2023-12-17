use clap::Parser;

use crate::args::Cli;

mod args;
mod constants;
mod individual;
mod simulation;
mod target;

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
}
