use clap::Parser;

use crate::args::Cli;

mod args;

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
}
