// use crate::{
//     get_content, get_reader, process_text_key_generate, process_text_sign, process_text_verify,
//     CmdExecutor,
// };

//cbtmark 这里要模拟输入 echo -n "baotao" 否则签名有问题，可能是因为换行符的问题
// cbtmark echo -n "baotao" 不会换行 这里要使用 echo -n 否则会换行
// cbtmark echo  "baotao" 会换行

/*
    cargo run -- text sign -k ./fixtures/blake3.txt

    输入baotao 结果是 2TsEXzJZTNFxlI0dIsNbgHEcrzT0vXVPX1LMKp7Ql9c


    cargo run -- text verify -k ./fixtures/blake3.txt --sig 2TsEXzJZTNFxlI0dIsNbgHEcrzT0vXVPX1LMKp7Ql9c
    输入baotao 结果是 true



    echo -n "baotao" | cargo run text sign --format blake3 --key fixtures/blake3.txt
 */

use super::{ verify_file, verify_path };
use base64::{ engine::general_purpose::URL_SAFE_NO_PAD, Engine };
use clap::Parser;
use std::{ fmt, path::PathBuf, str::FromStr };

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")] Sign(
        TextSignOpts,
    ),
    #[command(about = "Verify a signature with a public/session key")] Verify(TextVerifyOpts),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")] Generate(
        KeyGenerateOpts,
    ),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

//
