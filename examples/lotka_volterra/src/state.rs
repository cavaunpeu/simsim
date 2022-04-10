use gds::state;

#[derive(Debug)]
pub struct State {
  pub population_size: f64,
  pub food_supply: f64,
}

impl state::BaseState for State {}