use csv;
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
        output_path: String,
    ) -> Result<(), csv::Error> {
        let results = self._run(runs, steps_per_run);
        let mut writer = csv::Writer::from_path(output_path)?;
        if let Some((state, _run, _step)) = results.first() {
            let mut keys = vec!["run", "step"];
            // Get data columns; assumed to be the same for all records.
            let cols = state.get_serializable_record().keys().cloned().collect::<Vec<&str>>();
            keys.extend(cols);
            // Write .csv header.
            writer.write_record(&keys)?;
            // Append simulation results.
            for (state, run, step) in results.iter() {
                let mut record = state.get_serializable_record();
                record.insert("run", (*run).into());
                record.insert("step", (*step).into());
                let vals = keys.iter().map(|k| record.get(*k).unwrap().to_string());
                writer.write_record(vals)?;
            }
        }
        writer.flush()?;
        Ok(())
    }

    fn _run(
        &self,
        runs: u32,
        steps_per_run: u32
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
