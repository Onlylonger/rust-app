use std::path::Path;

use crate::{csv, password};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "slr", version, about = "a simple cli tool for daily function with rust", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: SubCmd,
}

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(csv::CsvOpts),
    #[command(name = "gp", about = "Password Generator")]
    GenPassword(password::GenPasswordOpts),
}

pub fn file_check(filename: &str) -> anyhow::Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("file cannot found!")
    }
}
