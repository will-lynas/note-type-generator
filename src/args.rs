use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf)
    )]
    pub css: PathBuf,

    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf)
    )]
    pub template: PathBuf,

    #[arg(
        long,
        value_hint = ValueHint::FilePath,
        value_parser = clap::value_parser!(PathBuf)
    )]
    pub config: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
