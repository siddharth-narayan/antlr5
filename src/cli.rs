use std::path::PathBuf;

use tracing::level_filters::LevelFilter;
use clap::{Command, ValueEnum, arg, command};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set the logging level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    pub log_level: LevelFilter,

    pub path: PathBuf
}