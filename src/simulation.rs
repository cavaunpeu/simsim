use std::marker::PhantomData;

use crate::{system::BaseSystem, state::State};

pub struct Simulation<U: State, T: BaseSystem<U>> {
  system: T,
  state: PhantomData<U>
}

impl<U: State, T: BaseSystem<U>> Simulation<U, T> {
  pub fn new(system: T) -> Self {
    Simulation {
      system,
      state: PhantomData {}
    }
  }

  pub fn run(&self, runs: i32, steps_per_run: i32, _visualize_results: bool) -> Vec<(U, i32, i32)> {
    let mut results = Vec::<(U, i32, i32)>::new();
    for run in 0..runs {
      let mut history = Vec::<U>::new();
      let mut index = Vec::<(i32, i32)>::new();
      for step in 0..steps_per_run {
        let state = match step {
          0 => self.system.initial_step(),
          _ => self.system.step(&history[history.len() - 1], &history),
        };
        index.push((run, step));
        history.push(state);
      }
      for (state, (run, step)) in history.into_iter().zip(index) {
        results.push((state, run, step));
      }
    }
    results
  }
}