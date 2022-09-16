use clap;

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(long = "runs")]
    pub runs: u32,
    #[clap(long = "steps_per_run")]
    pub steps_per_run: u32,
    #[clap(long = "output_dir")]
    pub output_dir: String,
    #[clap(long = "configs_path")]
    pub configs_path: String,
}
