use std::collections::HashMap;

use crate::state::BaseState;

pub trait BaseSystem<T: BaseState> {
    fn from_config(config: &HashMap<String, f64>) -> Self;

    fn initial_step(&mut self) -> T;

    fn step(&mut self, state: &T, history: &Vec<T>) -> T;

    fn get_params(&self) -> HashMap<&'static str, f64>;
}
