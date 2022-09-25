use std::collections::HashMap;

use simsim::system::BaseSystem;
use crate::io::{Config, State};


pub struct LorenzSystem {
  x: f64,
  y: f64,
  z: f64,
  sigma: f64,
  rho: f64,
  beta: f64,
}

impl BaseSystem<Config, State> for LorenzSystem {
  fn from_config(config: &Config) -> Self {
    LorenzSystem {
      x: config.x,
      y: config.y,
      z: config.z,
      sigma: config.sigma,
      rho: config.rho,
      beta: config.beta
    }
  }

  fn initial_step(&mut self) -> State {
    State {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }

  fn step(&mut self, state: &State, _history: &Vec<State>) -> State {
    let dx = self.sigma * (state.y - state.x);
    let dy = state.x * (self.rho - state.z) - state.y;
    let dz = state.x * state.y - self.beta * state.z;
    State {
      x: state.x + dx,
      y: state.y + dy,
      z: state.z + dz,
    }
  }

  fn get_params(&self) -> HashMap<&'static str, f64> {
    HashMap::from([
        ("sigma", self.sigma),
        ("beta", self.beta),
        ("rho", self.rho),
    ])
  }
}