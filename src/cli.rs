use std::path::PathBuf;

use clap::{ArgAction::Count, Parser};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, value_name = "INDEX")]
    pub mirror: Option<usize>,
    #[arg(short, long, value_name = "NUMBER")]
    pub threads: Option<usize>,
    #[arg(short, long, value_name = "DIR")]
    pub path: Option<PathBuf>,
    #[arg(short, long, action = Count)]
    pub beta: usize,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
