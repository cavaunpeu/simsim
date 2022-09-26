use serde::{Deserialize, Serialize};
use crate::system::LorenzSystem;

#[derive(Debug, Serialize)]
pub struct State {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl From<&mut LorenzSystem> for State {
    fn from(system: &mut LorenzSystem) -> Self {
        let LorenzSystem { x, y, z, .. } = system;
        Self { x: *x, y: *y, z: *z }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub sigma: f64,
    pub beta: f64,
    pub rho: f64
}