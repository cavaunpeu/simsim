pub mod system;
pub mod state;

use std::process;
use clap::Parser;
use crate::system::LotkaVolterraSystem;
use simsim::{simulation::Simulation, cli::Cli};


fn main() {
    let args = Cli::parse();
    let system = system::LotkaVolterraSystem::new(
        50.0,
        1000.0,
        0.01,
        0.01
    );
    let simulation = Simulation::<state::State, LotkaVolterraSystem>::new(system);
    if let Err(e) = simulation.run(args.runs, args.steps_per_run, args.output_dir) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
