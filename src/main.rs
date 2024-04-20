use clap::Parser;
// slr csv -i ccc.csv
use rust_app::{process_csv, Args, SubCommand};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        SubCommand::Csv(opts) => process_csv(&opts)?,
    }
    Ok(())
}
