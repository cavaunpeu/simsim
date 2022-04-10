pub mod system;
pub mod state;

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
    let _results = simulation.run(args.runs, args.steps_per_run, args.viz_results);
}
