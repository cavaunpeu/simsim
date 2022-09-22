use std::collections::HashMap;
use serde_json::{Value, Map};

use crate::state::BaseState;

pub trait BaseSystem<T: BaseState> {
    fn from_config(config: &Map<String, Value>) -> Self;

    fn initial_step(&mut self) -> T;

    fn step(&mut self, state: &T, history: &Vec<T>) -> T;

    fn get_params(&self) -> HashMap<&'static str, f64>;
}
