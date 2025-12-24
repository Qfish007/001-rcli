mod opts;
mod process;

pub use opts::{CsvOpts, Opts, SubCommand};
pub use process::process_csv;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_test() {}
}
