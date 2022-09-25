pub mod system;
pub mod io;

use clap::Parser;
use std::error::Error;
use simsim::{cli::Cli, simulation::Simulation};


fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let simulation = Simulation::<io::Config, io::State, system::LotkaVolterraSystem>::from_configs_path(args.configs_path);
    simulation.run(args.runs, args.steps_per_run, args.output_dir)?;
    Ok(())
}
