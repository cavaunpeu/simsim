use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct State {
  pub x: f64,
  pub y: f64,
  pub z: f64,
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