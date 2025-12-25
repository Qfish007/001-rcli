use std::fs;

use crate::OutputFormat;
use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[allow(unused)]
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

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader: Reader<std::fs::File> = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(100);

    // let bind_struct = || -> Result<()> {
    //     for result in reader.deserialize() {
    //         let record: Player = result?;
    //         println!("{:#?}", record);
    //         ret.push(record);
    //     }
    //     Ok(())
    // };

    let mut no_bind_struct = || -> Result<()> {
        let headers = reader.headers()?.clone();
        for result in reader.records() {
            let record = result?;

            let json_value = headers.iter().zip(record.iter()).collect::<Value>();
            ret.push(json_value);
        }

        Ok(())
    };

    no_bind_struct()?;

    // let json = serde_json::to_string_pretty(&ret)?;

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    fs::write(output, content)?;

    Ok(())
}
// {
