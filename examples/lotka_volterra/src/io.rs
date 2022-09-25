use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct State {
  pub population_size: f64,
  pub food_supply: f64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub population_size: f64,
    pub food_supply: f64,
    pub reproduction_rate: f64,
    pub consumption_rate: f64
}