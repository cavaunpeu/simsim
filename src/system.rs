use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub trait BaseSystem<I: for<'a> Deserialize<'a>, O: Serialize> {
    fn from_config(config: &I) -> Self;

    fn initial_step(&mut self) -> O;

    fn step(&mut self, state: &O, history: &Vec<O>) -> O;

    fn get_params(&self) -> HashMap<&'static str, f64>;
}
