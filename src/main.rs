use clap::Parser;
use rcli::process_csv;
use rcli::Opts;
use rcli::SubCommand;

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    print!("{:#?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
