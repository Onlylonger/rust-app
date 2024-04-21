use clap::Parser;

use rust_app::{common, csv, password};

fn main() -> anyhow::Result<()> {
    let args = common::Args::parse();

    match args.cmd {
        common::SubCmd::Csv(opts) => csv::process(&opts)?,
        common::SubCmd::GenPassword(opts) => password::process(&opts)?,
    }

    Ok(())
}
