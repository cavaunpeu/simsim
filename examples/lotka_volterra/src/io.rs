use serde::{Deserialize, Serialize};
use crate::system::LotkaVolterraSystem;

#[derive(Debug, Serialize)]
pub struct State {
  pub population_size: f64,
  pub food_supply: f64,
}

impl From<&mut LotkaVolterraSystem> for State {
    fn from(system: &mut LotkaVolterraSystem) -> Self {
        let LotkaVolterraSystem { population_size, food_supply, .. } = system;
        State { population_size: *population_size, food_supply: *food_supply }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub population_size: f64,
    pub food_supply: f64,
    pub reproduction_rate: f64,
    pub consumption_rate: f64
}