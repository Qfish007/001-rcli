mod opts;
mod process;

pub use opts::{CsvOpts, GenPassOpts, Opts, OutputFormat, SubCommand};
pub use process::{process_csv, process_genpass};

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_test() {}
}
