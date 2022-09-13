use std::collections::HashMap;
use simsim::state;

#[derive(Debug)]
pub struct State {
  pub population_size: f64,
  pub food_supply: f64,
}

impl state::BaseState for State {
    fn get_serializable_record(&self) -> HashMap<&str, f64> {
        HashMap::from([
            ("population_size", self.population_size),
            ("food_supply", self.food_supply),
        ])
    }
}