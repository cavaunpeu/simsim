use std::collections::HashMap;

use crate::state::BaseState;

pub trait BaseSystem<T: BaseState> {
    fn from_config(config: HashMap<String, f64>) -> Self;

    fn initial_step(&self) -> T;

    fn step(&self, state: &T, history: &Vec<T>) -> T;

    fn get_system_params(&self) -> HashMap<&str, f64>;
}
