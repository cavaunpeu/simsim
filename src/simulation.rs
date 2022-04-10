use std::marker::PhantomData;

use crate::{state::BaseState, system::BaseSystem};

pub struct Simulation<U: BaseState, T: BaseSystem<U>> {
    system: T,
    state: PhantomData<U>,
}

impl<U: BaseState, T: BaseSystem<U>> Simulation<U, T> {
    pub fn new(system: T) -> Self {
        Simulation {
            system,
            state: PhantomData {},
        }
    }

    pub fn run(
        &self,
        runs: u32,
        steps_per_run: u32,
        _visualize_results: bool,
    ) -> Vec<(U, u32, u32)> {
        let mut results = Vec::<(U, u32, u32)>::new();
        for run in 0..runs {
            let mut history = Vec::<U>::new();
            let mut index = Vec::<(u32, u32)>::new();
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
