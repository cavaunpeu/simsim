use std::collections::HashMap;

use simsim::system::BaseSystem;
use crate::io::{Config, State};


pub struct LorenzSystem {
  pub x: f64,
  pub y: f64,
  pub z: f64,
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

  fn step(&mut self, _state: &State, _history: &Vec<State>) -> State {
    let dx = self.sigma * (self.y - self.x);
    let dy = self.x * (self.rho - self.z) - self.y;
    let dz = self.x * self.y - self.beta * self.z;
    self.x += dx;
    self.y += dy;
    self.z += dz;
    State::from(self)
  }

  fn get_params(&self) -> HashMap<&'static str, f64> {
    HashMap::from([
        ("sigma", self.sigma),
        ("beta", self.beta),
        ("rho", self.rho),
    ])
  }
}