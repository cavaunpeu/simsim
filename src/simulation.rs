use csv;
use std::{error::Error, collections::HashMap};
use std::fs;
use std::marker::PhantomData;

use crate::{state::BaseState, system::BaseSystem};

pub struct Simulation<U: BaseState, T: BaseSystem<U>> {
    configs: Vec<HashMap<String, f64>>,
    system: PhantomData<T>,
    state: PhantomData<U>
}

impl<U: BaseState, T: BaseSystem<U>> Simulation<U, T> {
    pub fn from_configs(configs: Vec<HashMap<String, f64>>) -> Self {
        Simulation {
            configs,
            system: PhantomData {},
            state: PhantomData {},
        }
    }

    pub fn run(&self, runs: u32, steps_per_run: u32, output_dir: String) -> Result<(), Box<dyn Error>> {
        let num_rows = self.configs.len() as u32 * runs * steps_per_run;
        let mut results = Vec::with_capacity(num_rows as usize);
        let mut params = Vec::with_capacity(self.configs.len());
        fs::create_dir_all(&output_dir)?;
        for (idx, config) in (&self.configs).iter().enumerate() {
            let run = SingleSimulationRun::<U, T>::from_config(config);
            let res = run.run(runs, steps_per_run);
            let prm = run.system.get_params();
            params.push((idx, prm));
            results.extend(res);
        }
        self.write_results_to_csv(results, &output_dir)?;
        self.write_params_to_csv(params, &output_dir)?;
        Ok(())
    }

    fn write_params_to_csv(&self, mut params: Vec<(usize, HashMap<&str, f64>)>, output_dir: &str) -> Result<(), csv::Error> {
        if let Some((_idx, prm)) = params.first() {
            let params_path = format!("{}/params.csv", output_dir);
            let mut writer = csv::Writer::from_path(params_path)?;
            let mut keys = vec!["config_idx"];
            // Get param keys; assumed to be the same for all configs.
            let cols = prm.keys().cloned().collect::<Vec<&str>>();
            keys.extend(cols);
            // Write .csv header.
            writer.write_record(&keys)?;
            // Append config params.
            for (idx, prm) in params.iter_mut() {
                prm.insert("config_idx", *idx as f64);
                let vals = keys.iter().map(|k| prm.get(*k).unwrap().to_string());
                writer.write_record(vals)?;
            }
            writer.flush()?;
        }
        Ok(())
    }

    fn write_results_to_csv(&self, results: Vec<(U, u32, u32)>, output_dir: &str) -> Result<(), csv::Error> {
        if let Some((state, _run, _step)) = results.first() {
            let results_path = format!("{}/results.csv", output_dir);
            let mut writer = csv::Writer::from_path(results_path)?;
            let mut keys = vec!["run", "step"];
            // Get data keys; assumed to be the same for all records.
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
            writer.flush()?;
        }
        Ok(())
    }
}

struct SingleSimulationRun<U: BaseState, T: BaseSystem<U>> {
    system: T,
    state: PhantomData<U>
}

impl<U: BaseState, T: BaseSystem<U>> SingleSimulationRun<U, T> {
    fn from_config(config: &HashMap<String, f64>) -> Self {
        SingleSimulationRun {
            system: T::from_config(config),
            state: PhantomData {}
        }
    }

    fn run(
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