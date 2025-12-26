use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    print!("{:#?}", opts);

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
        SubCommand::Base64(opts) => {
            // TODO: implement base64 command
            println!("base64 command not implemented yet, {:?}", opts);
        }
    }

    Ok(())
}
