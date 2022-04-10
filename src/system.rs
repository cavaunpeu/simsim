use crate::state::BaseState;

pub trait BaseSystem<T: BaseState> {
  fn initial_step(&self) -> T;

  fn step(&self, state: &T, history: &Vec<T>) -> T;
}