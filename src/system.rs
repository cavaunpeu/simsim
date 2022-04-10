use crate::state::State;

pub trait BaseSystem<T: State> {
  fn initial_step(&self) -> T;

  fn step(&self, state: &T, history: &Vec<T>) -> T;
}