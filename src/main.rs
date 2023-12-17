use clap::Parser;

use crate::args::Cli;

mod args;
mod constants;
mod individual;
mod target;

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
}
