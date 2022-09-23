use std::collections::HashMap;
use serde_json::{Value, Map};

use simsim::system::BaseSystem;
use crate::state::State;


pub struct LotkaVolterraSystem {
  population_size: f64,
  food_supply: f64,
  reproduction_rate: f64,
  consumption_rate: f64,
}

impl BaseSystem<State> for LotkaVolterraSystem {
  fn from_config(config: &Map<String, Value>) -> Self {
    LotkaVolterraSystem {
      population_size: config["population_size"].as_f64().unwrap(),
      food_supply: config["food_supply"].as_f64().unwrap(),
      reproduction_rate: config["reproduction_rate"].as_f64().unwrap(),
      consumption_rate: config["consumption_rate"].as_f64().unwrap(),
    }
  }

  fn initial_step(&mut self) -> State {
    State {
      population_size: self.population_size,
      food_supply: self.food_supply,
    }
  }

  fn step(&mut self, state: &State, _history: &Vec<State>) -> State {
    State {
      population_size: (state.population_size + self.reproduction_rate * state.food_supply).max(0.0),
      food_supply: (state.food_supply - self.consumption_rate * state.population_size).max(0.0)
    }
  }

  fn get_params(&self) -> HashMap<&'static str, f64> {
    HashMap::from([
        ("reproduction_rate", self.reproduction_rate),
        ("consumption_rate", self.consumption_rate),
    ])
  }
}