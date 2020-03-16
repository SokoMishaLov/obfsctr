use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::Read;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "sokomishalov")]
struct Opts {
    /// Sets an input file or directory
    #[clap(short = "i", long = "input")]
    input: String,

    /// Sets an output file or directory (current directory by default)
    #[clap(short = "o", long = "output", default_value = ".")]
    output: String,

    /// Sets a number of worker threads (4 by default)
    #[clap(short = "n", long = "n-threads", default_value = "4")]
    threads: u8,

    /// A level of verbosity, and can be used multiple times
    #[clap(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,
}

impl Display for Opts {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "input:{}, output:{} threads:{} verbose:{}", self.input, self.output, self.threads, self.verbose)
    }
}


fn main() {
    let opts: Opts = Opts::parse();

    println!("{}", opts);
}
