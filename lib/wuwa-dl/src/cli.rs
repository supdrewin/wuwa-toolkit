use std::path::PathBuf;

use clap::{ArgAction::SetTrue, Parser};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, value_name = "INDEX")]
    pub mirror: Option<usize>,

    #[arg(short, long, value_name = "NUMBER")]
    pub threads: Option<usize>,

    #[arg(short, long, value_name = "DIR")]
    pub path: Option<PathBuf>,

    #[arg(short, long, action = SetTrue)]
    pub global: bool,

    #[arg(short, long, action = SetTrue)]
    pub beta: bool,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
