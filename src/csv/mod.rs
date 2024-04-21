use std::{fs, path::Path, str::FromStr};

use clap::Parser;

use crate::common;

#[derive(Debug, Clone)]
enum Format {
    Json,
    Yaml,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            _ => Err(anyhow::anyhow!("Unsupported Format Type!")),
        }
    }
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = common::file_check)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(long, default_value = "dist")]
    pub outdir: String,

    #[arg(long, default_value_t = true)]
    header: bool,

    #[arg(long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value = "json")]
    format: Format,
}

pub fn process(opts: &CsvOpts) -> anyhow::Result<()> {
    let mut rdr = csv_::Reader::from_path(&opts.input)?;
    let mut buf = Vec::with_capacity(128);

    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let val = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        buf.push(val);
    }

    let (content, extension) = match opts.format {
        Format::Json => (serde_json::to_string_pretty(&buf)?, "json"),
        Format::Yaml => (serde_yaml::to_string(&buf)?, "yaml"),
    };

    let out_dir = Path::new(&opts.outdir);
    let output = {
        let filename = {
            if let Some(output_filename) = &opts.output {
                Path::new(output_filename)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            } else {
                Path::new(&opts.input)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            }
        };
        let mut filename: String = filename
            .split('.')
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_string();
        filename.push('.');
        filename.push_str(extension);
        out_dir.join(filename)
    };

    fs::create_dir_all(out_dir)?;
    fs::write(output, content)?;

    eprintln!("Output Successful!");
    Ok(())
}
