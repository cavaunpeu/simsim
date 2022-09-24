use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct State {
  pub population_size: f64,
  pub food_supply: f64,
}