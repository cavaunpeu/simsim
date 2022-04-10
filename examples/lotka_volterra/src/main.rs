pub mod system;
pub mod state;
use gds::simulation::Simulation;

use crate::system::LotkaVolterraSystem;

fn main() {
    let system = system::LotkaVolterraSystem::new(
        50.0,
        1000.0,
        0.01,
        0.01
    );
    let simulation = Simulation::<state::State, LotkaVolterraSystem>::new(system);
    let results = simulation.run(3, 5, false);
}
