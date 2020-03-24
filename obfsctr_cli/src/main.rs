use std::{
    fs,
    fs::metadata,
    io,
    io::{Error, ErrorKind},
    path::Path,
    path::PathBuf,
    str,
    sync::mpsc::channel,
    time::Instant,
};

use clap::Clap;
use colored::*;
use fasthash::murmur3;
use regex::Regex;
use threadpool::ThreadPool;

use obfsctr_core::regex_obfsctr::RegexObfuscator;

const SEED: u32 = 10071995;
const MARKER: &str = "**";

#[derive(Clap)]
#[clap(version = "0.1.0", author = "sokomishalov")]
struct Opts {
    /// Sets an input file or directory
    #[clap(short = "i", long = "input")]
    input: String,

    /// Sets a regex for
    #[clap(short = "r", long = "regex", default_value = "")]
    regex: String,

    /// Sets a number of worker threads (4 by default)
    #[clap(short = "n", long = "n-threads", default_value = "4")]
    threads: usize,
}

fn extract_file_paths(input_path: &str) -> io::Result<Vec<PathBuf>> {
    let path = Path::new(input_path);
    let md = metadata(&path)?;

    if md.is_dir() {
        let entries: Vec<PathBuf> = fs::read_dir(path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        Ok(entries)
    } else if md.is_file() {
        Ok(vec![path.to_path_buf()])
    } else {
        Err(Error::new(ErrorKind::NotFound, "File or directory does not exist!"))
    }
}

fn replacer(raw: &str) -> String {
    let val = murmur3::hash128_with_seed(&raw, SEED);
    transform_u128_array_to_str(val)
}

fn transform_u128_array_to_str(x: u128) -> String {
    format!("{}{}{}", MARKER, x.to_string(), MARKER)
}

fn main() {
    let opts: Opts = Opts::parse();

    let thread_pool = ThreadPool::new(opts.threads);
    let file_paths = extract_file_paths(opts.input.as_str()).unwrap();
    let re = Regex::new(opts.regex.as_str()).unwrap();

    let (tx, rx) = channel();
    for path in file_paths.clone() {
        let tx = tx.clone();
        let re = re.clone();
        thread_pool.execute(move || {
            path.as_path().obfuscate(&re, replacer);
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }

    let now = Instant::now();
    rx.iter().take(file_paths.len()).all(|_| { true });

    println!("{} {:?}", "Total time:".green(), now.elapsed());
}