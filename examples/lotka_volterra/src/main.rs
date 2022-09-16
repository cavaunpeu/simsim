pub mod system;
pub mod state;

use clap::Parser;
use std::error::Error;
use simsim::{cli::Cli, util::load_configs, simulation::Simulation};


fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let configs = load_configs(args.configs_path)?;
    let simulation = Simulation::<state::State, system::LotkaVolterraSystem>::from_configs(configs);
    simulation.run(args.runs, args.steps_per_run, args.output_dir)?;
    Ok(())
}
