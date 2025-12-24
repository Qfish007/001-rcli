use std::path::Path;
use clap::Parser;
use csv::Reader;
use serde::{ Serialize, Deserialize }; // use serde_json::Value;
use anyhow::Result;
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")] Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    // #[serde(rename = "Name")]
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    print!("{:#?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader: Reader<std::fs::File> = Reader::from_path(opts.input)?;

            for result in reader.deserialize() {
                let record: Player = result?;
                println!("{:#?}", record);
            }
        }
    }

    Ok(())
}
