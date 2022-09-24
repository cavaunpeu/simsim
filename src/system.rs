use std::collections::HashMap;
use serde::Serialize;
use serde_json::{Value, Map};

pub trait BaseSystem<T: Serialize> {
    fn from_config(config: &Map<String, Value>) -> Self;

    fn initial_step(&mut self) -> T;

    fn step(&mut self, state: &T, history: &Vec<T>) -> T;

    fn get_params(&self) -> HashMap<&'static str, f64>;
}
