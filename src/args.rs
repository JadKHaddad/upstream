use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Config file: The path to the configuration file
    #[clap(long, default_value = "config.yaml")]
    pub config_file: PathBuf,
}
