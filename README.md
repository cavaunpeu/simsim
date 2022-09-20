# What is simsim?

simsim is a framework for (sim)ple (sim)ulation of generalized dynamical systems. A user implements a system, a state transition function, and a specification of the data they'd like to record, and simsim runs this simulation—and structures its results—in a straightforward manner.

# How do I implement a simulation?

1. Implement a system struct that inherits from the `BaseSystem` [trait](https://github.com/cavaunpeu/simsim/blob/8b66bda7bfcad9a8581dfe27186200ad4e1336c1/src/system.rs#L5).
2. Implement a state struct that inherits from the `BaseState` [trait](https://github.com/cavaunpeu/simsim/blob/882071ca7152c2312b1a2dbaf0e5af742cbbdfc7/src/state.rs#L3).
3. Given the above, instantiate a `Simulation` [object](https://github.com/cavaunpeu/simsim/blob/8b66bda7bfcad9a8581dfe27186200ad4e1336c1/src/simulation.rs#L8).
4. Call the `Simulation`'s `run` method.

For example, for a [Lotka-Volterra](https://en.wikipedia.org/wiki/Lotka%E2%80%93Volterra_equations) simulation, these steps are respectively implemented here:

1. [System](https://github.com/cavaunpeu/simsim/blob/8b66bda7bfcad9a8581dfe27186200ad4e1336c1/examples/lotka_volterra/src/system.rs#L7).
2. [State](https://github.com/cavaunpeu/simsim/blob/882071ca7152c2312b1a2dbaf0e5af742cbbdfc7/examples/lotka_volterra/src/state.rs#L5).
3. [Simulation](https://github.com/cavaunpeu/simsim/blob/2ef437c6c1a57c623d2f8064b8c66799b023e366/examples/lotka_volterra/src/main.rs#L12).
4. [Run](https://github.com/cavaunpeu/simsim/blob/2ef437c6c1a57c623d2f8064b8c66799b023e366/examples/lotka_volterra/src/main.rs#L13).

# How do I run a simulation?

To run the Lotka-Volterra simulation above, run the following commands:

```
git clone git@github.com:cavaunpeu/simsim.git
cargo run \
  --manifest-path examples/lotka_volterra/Cargo.toml \
  -- \
  --runs 10 \
  --steps_per_run 5 \
  --output_dir output \
  --configs_path examples/lotka_volterra/configs.json
```

# Where do the configs come from?

In simsim, you create your own (JSON) configs—however you see fit. For instance, you can do this manually, or programmatically in a language like Python.

We hope that forcing the user to explicitly provide all configs they desire to run simplifies the experience of using this tool.

# What does simsim output?

Presently, we write the following two files to your specified `--output_dir`:

1. `results.csv`: A .csv containing all of your `State`'s `(key, val)` pairs in each `step` of each `run`.
2. `params.csv`: Those `(key, val)` pairs in your config that you [designate](https://github.com/cavaunpeu/simsim/blob/8b66bda7bfcad9a8581dfe27186200ad4e1336c1/src/system.rs#L12) as system-level parameters. Nominally, these are constants that characterize a given configuration.

# How should I get started?

Simply, we recommend duplicating the full Lotka-Volterra [example](https://github.com/cavaunpeu/simsim/tree/main/examples/lotka_volterra) then replacing its code with your own.

Additionally, if you are working from a fresh repository, you can specify the `simsim` dependency in your `Cargo.toml` as such:

```
[dependencies]
simsim = { git="https://github.com/cavaunpeu/simsim.git" }
```

# Questions?

Please open an [issue](https://github.com/cavaunpeu/simsim/issues), or DM me on [Twitter](https://twitter.com/willwolf_).