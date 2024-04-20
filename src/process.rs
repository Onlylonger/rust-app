use clap::{Parser, Subcommand};

use crate::CsvOpts;

#[derive(Parser, Debug)]
#[command(name = "slr", version, about = "a simple cli tool for daily function with rust", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}
