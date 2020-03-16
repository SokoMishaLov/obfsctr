use clap::Clap;

use obfsctr_core::obfsctr::obfuscate;

#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K.")]
struct Opts {
    /// Sets an input file or directory
    #[clap(short = "i", long = "input")]
    input: String,

    /// Sets an output file or directory
    #[clap(short = "o", long = "output")]
    output: String,

    /// Sets a number of worker threads
    #[clap(short = "n", long = "n-threads")]
    threads: u8,

    /// A level of verbosity, and can be used multiple times
    #[clap(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,
}


fn main() {
    let opts: Opts = Opts::parse();

    let val: String = obfuscate(String::from("todo"));

    println!("{}", val);
}
