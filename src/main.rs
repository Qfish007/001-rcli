use clap::Parser;
use rcli::process_csv;
use rcli::Opts;
use rcli::SubCommand;

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
    }

    Ok(())
}
