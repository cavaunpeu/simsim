use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::{error::Error, collections::HashMap};
use std::fs;
use std::marker::PhantomData;

use kdam::{tqdm, BarExt};

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

pub struct Simulation<I: for<'de> Deserialize<'de>, O: Serialize, S: BaseSystem<I, O>> {
    configs: Vec<I>,
    system: PhantomData<S>,
    state: PhantomData<O>
}

impl<I: for<'de> Deserialize<'de>, O: Serialize, S: BaseSystem<I, O>> Simulation<I, O, S> {
    pub fn from_configs_path(path: String) -> Self {
        let data = fs::read_to_string(&path).expect(&format!("{} not found", &path));
        let configs: Vec<I> = serde_json::from_str(&data).expect("Could not read JSON");
        Simulation {
            configs,
            system: PhantomData {},
            state: PhantomData {},
        }
    }

    pub fn run(&self, runs: u32, steps_per_run: u32, output_dir: String) -> Result<(), Box<dyn Error>> {
        let num_runs = self.configs.len() * runs as usize;
        let num_rows = num_runs * steps_per_run as usize;
        let mut results = Vec::<Record<O>>::with_capacity(num_rows);
        let mut params = Vec::<Params>::with_capacity(self.configs.len());
        let mut pb = tqdm!(total = num_runs);
        fs::create_dir_all(&output_dir)?;
        for (config_idx, config) in (0u32..).zip(&self.configs) {
            for run in 0..runs {
                let mut sim = SingleSimulationRun::<I, O, S>::from_config(config);
                if run == 0 {
                    params.push(Params { config_idx, params: sim.system.get_params() });
                }
                results.extend(
                    sim.run(steps_per_run)
                    .into_iter()
                    .map(|(state, step)| Record { config_idx, run, step, state })
                    .collect::<Vec<_>>()
                );
                pb.update(1);
            }
        }
        println!("Writing results of {} runs to disk...", num_runs);
        self.write_results_to_json(results, &output_dir)?;
        self.write_params_to_json(params, &output_dir)?;
        Ok(())
    }

    fn write_results_to_json(&self, results: Vec<Record<O>>, output_dir: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/results.json", output_dir);
        let file = fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &results)?;
        writer.flush().expect("Writer failed to flush");
        Ok(())
    }

    fn write_params_to_json(&self, params: Vec<Params>, output_dir: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/params.json", output_dir);
        let file = fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &params)?;
        writer.flush().expect("Writer failed to flush");
        Ok(())
    }
}

struct SingleSimulationRun<I: for<'de> Deserialize<'de>, O: Serialize, S: BaseSystem<I, O>> {
    system: S,
    state: PhantomData<O>,
    config: PhantomData<I>
}

impl<I: for<'de> Deserialize<'de>, O: Serialize, S: BaseSystem<I, O>> SingleSimulationRun<I, O, S> {
    fn from_config(config: &I) -> Self {
        SingleSimulationRun {
            system: S::from_config(config),
            state: PhantomData {},
            config: PhantomData {}
        }
    }

    fn run(
        &mut self,
        steps_per_run: u32
    ) -> Vec<(O, u32)> {
        let mut results = Vec::<(O, u32)>::new();
        let mut history = Vec::<O>::new();
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