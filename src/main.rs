use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_key_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, SubCommand, TextSubCommand,
};
use std::fs;
mod utils;
use crate::utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    // println!("opts:{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };

            println!("{:?}", output);
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", password);
        }
        SubCommand::Base64(b64cmd) => match b64cmd {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(opts.input.as_str())?;
                let result = process_encode(&mut reader, opts.format)?;
                println!("{}", result);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(opts.input.as_str())?;
                println!("decode:{:#?} ", process_decode(&mut reader, opts.format)?);
            }
        },
        SubCommand::Text(textcmd) => match textcmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(opts.input.as_str())?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                println!("{:?}", URL_SAFE_NO_PAD.encode(sig));
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(opts.input.as_str())?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let result = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                println!("result:{:?}", result);
            }
            TextSubCommand::Generate(opts) => {
                let map = process_text_key_generate(opts.format)?;
                for (k, v) in map {
                    let _ = fs::write(opts.output_path.join(k), v);
                }
            }
        },
        SubCommand::Http(httpcmd) => match httpcmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }

    Ok(())
}

#[allow(unused)]
fn test() {
    let key: &[u8] = "1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let key: [u8; 8] = (&key[..8]).try_into().unwrap();
    println!("key:{:?}", key)
}
