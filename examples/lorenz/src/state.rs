use std::collections::HashMap;
use simsim::state;

#[derive(Debug)]
pub struct State {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl state::BaseState for State {
    fn get_serializable_record(&self) -> HashMap<&str, f64> {
        HashMap::from([
            ("x", self.x),
            ("y", self.y),
            ("z", self.z),
        ])
    }
}