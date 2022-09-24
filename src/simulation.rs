use serde::Serialize;
use serde_json::{Value, Map};
use std::{error::Error, collections::HashMap};
use std::fs;
use std::marker::PhantomData;

use crate::system::BaseSystem;

#[derive(Serialize)]
pub struct Record<T: Serialize> {
    config_idx: u32,
    run: u32,
    step: u32,
    state: T
}

#[derive(Serialize)]
pub struct Params {
    config_idx: u32,
    params: HashMap<&'static str, f64>
}

pub struct Simulation<U: Serialize, T: BaseSystem<U>> {
    configs: Vec<Map<String, Value>>,
    system: PhantomData<T>,
    state: PhantomData<U>
}

impl<U: Serialize, T: BaseSystem<U>> Simulation<U, T> {
    pub fn from_configs(configs: Vec<Map<String, Value>>) -> Self {
        Simulation {
            configs,
            system: PhantomData {},
            state: PhantomData {},
        }
    }

    pub fn run(&self, runs: u32, steps_per_run: u32, output_dir: String) -> Result<(), Box<dyn Error>> {
        let num_rows = self.configs.len() as u32 * runs * steps_per_run;
        let mut results = Vec::<Record<U>>::with_capacity(num_rows as usize);
        let mut params = Vec::<Params>::with_capacity(self.configs.len());
        fs::create_dir_all(&output_dir)?;
        for (config_idx, config) in (0u32..).zip(&self.configs) {
            for run in 0..runs {
                let mut sim = SingleSimulationRun::<U, T>::from_config(config);
                if run == 0 {
                    params.push(Params { config_idx, params: sim.system.get_params() });
                }
                results.extend(
                    sim.run(steps_per_run)
                    .into_iter()
                    .map(|(state, step)| Record { config_idx, run, step, state })
                    .collect::<Vec<_>>()
                );
            }
        }
        self.write_results_to_json(results, &output_dir)?;
        self.write_params_to_json(params, &output_dir)?;
        Ok(())
    }

    fn write_results_to_json(&self, results: Vec<Record<U>>, output_dir: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/results.json", output_dir);
        let file = fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &results)?;
        Ok(())
    }

    fn write_params_to_json(&self, params: Vec<Params>, output_dir: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/params.json", output_dir);
        let file = fs::File::create(path)?;
        serde_json::to_writer_pretty(file, &params)?;
        Ok(())
    }
}

struct SingleSimulationRun<U: Serialize, T: BaseSystem<U>> {
    system: T,
    state: PhantomData<U>
}

impl<U: Serialize, T: BaseSystem<U>> SingleSimulationRun<U, T> {
    fn from_config(config: &Map<String, Value>) -> Self {
        SingleSimulationRun {
            system: T::from_config(config),
            state: PhantomData {}
        }
    }

    fn run(
        &mut self,
        steps_per_run: u32
    ) -> Vec<(U, u32)> {
        let mut results = Vec::<(U, u32)>::new();
        let mut history = Vec::<U>::new();
        let mut steps = Vec::<u32>::new();
        for step in 0..steps_per_run {
            let state = match step {
                0 => self.system.initial_step(),
                _ => self.system.step(&history[history.len() - 1], &history),
            };
            steps.push(step);
            history.push(state);
        }
        for (state, step) in history.into_iter().zip(steps) {
            results.push((state, step));
        }
        results
    }
}