use std::{fs, path::Path};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = file_check)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(long, default_value = "dist")]
    pub outdir: String,

    #[arg(long, default_value_t = true)]
    header: bool,

    #[arg(long, default_value_t = ',')]
    delimiter: char,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    index: String,
    #[serde(rename = "Customer Id")]
    customer_id: String,
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Last Name")]
    last_name: String,
    company: String,
    city: String,
    country: String,
    #[serde(rename = "Phone 1")]
    phone_1: String,
    #[serde(rename = "Phone 2")]
    phone_2: String,
    email: String,
    #[serde(rename = "Subscription Date")]
    subscription_date: String,
    website: String,
}

pub fn process_csv(opts: &CsvOpts) -> anyhow::Result<()> {
    let mut rdr = csv::Reader::from_path(&opts.input)?;
    let mut buf = Vec::with_capacity(128);
    for result in rdr.deserialize() {
        let record: Record = result?;
        buf.push(record);
    }

    let json = serde_json::to_string_pretty(&buf)?;
    let out_dir = Path::new(&opts.outdir);
    fs::create_dir_all(out_dir)?;
    fs::write(out_dir.join(&opts.output), json)?;
    Ok(())
}

fn file_check(filename: &str) -> anyhow::Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("file cannot found!")
    }
}
