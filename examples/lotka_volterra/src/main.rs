pub mod system;
pub mod state;

use clap::Parser;
use crate::system::LotkaVolterraSystem;

use simsim::{simulation::Simulation, cli::Cli, util::load_configs, system::BaseSystem};


fn main() {
    let args = Cli::parse();
    if let Ok(configs) = load_configs(args.configs_path) {
        for config in configs {
            let system = system::LotkaVolterraSystem::from_config(config);
            let _params = system.get_system_params();
            let simulation = Simulation::<state::State, LotkaVolterraSystem>::new(system);
            let _results = simulation.run(args.runs, args.steps_per_run);
        }
    }
}
