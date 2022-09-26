use std::collections::HashMap;

use simsim::system::BaseSystem;
use crate::io::{Config, State};


pub struct LotkaVolterraSystem {
  pub population_size: f64,
  pub food_supply: f64,
  reproduction_rate: f64,
  consumption_rate: f64,
}

impl BaseSystem<Config, State> for LotkaVolterraSystem {
  fn from_config(config: &Config) -> Self {
    LotkaVolterraSystem {
      population_size: config.population_size,
      food_supply: config.food_supply,
      reproduction_rate: config.reproduction_rate,
      consumption_rate: config.consumption_rate,
    }
  }

  fn initial_step(&mut self) -> State {
    State {
      population_size: self.population_size,
      food_supply: self.food_supply,
    }
  }

  fn step(&mut self, _state: &State, _history: &Vec<State>) -> State {
    self.population_size = (self.population_size + self.reproduction_rate * self.food_supply).max(0.0);
    self.food_supply = (self.food_supply - self.consumption_rate * self.population_size).max(0.0);
    State::from(self)
  }

  fn get_params(&self) -> HashMap<&'static str, f64> {
    HashMap::from([
        ("reproduction_rate", self.reproduction_rate),
        ("consumption_rate", self.consumption_rate),
    ])
  }
}