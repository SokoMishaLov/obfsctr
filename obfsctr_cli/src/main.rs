use std::{
    fs,
    path::Path,
    path::PathBuf,
    str,
    sync::mpsc::channel,
    time::Instant,
};

use clap::Clap;
use colored::*;
use fasthash::murmur3;
use regex::RegexSet;
use threadpool::ThreadPool;
use walkdir::WalkDir;

use obfsctr_core::regex_obfsctr::RegexObfuscator;

const SEED: u32 = 10071995;
const MARKER: &str = "**";

#[derive(Clap)]
#[clap(version = "0.1.0", author = "sokomishalov")]
struct Opts {
    /// Input file or directory
    #[clap(short = "i", long = "input")]
    input: String,

    /// Output directory
    #[clap(short = "o", long = "output")]
    output: String,

    /// Regular expression for the places to obfuscate
    #[clap(short = "r", long = "regex")]
    regex: String,

    /// Number of worker threads
    #[clap(short = "n", long = "n-threads", default_value = "4")]
    threads: usize,
}

pub fn main() {
    let opts: Opts = Opts::parse();

    let out = if opts.output.trim().is_empty() {
        &opts.input
    } else {
        &opts.output
    };
    let file_paths: Vec<(PathBuf, PathBuf)> = extract_file_paths_recursively(&opts.input, out);
    let re_set: RegexSet = RegexSet::new(&[opts.regex.as_str()]).expect("Invalid regular expression");
    let thread_pool: ThreadPool = ThreadPool::new(opts.threads);

    let (tx, rx) = channel();
    for (input_file_path, output_file_path) in file_paths.clone() {
        let tx = tx.clone();
        let re_set = re_set.clone();
        thread_pool.execute(move || {
            input_file_path.obfuscate(&mut output_file_path.clone(), &re_set, replacer);
            tx.send(1).expect("channel will be there waiting for the pool");
        });
    }

    let now = Instant::now();
    rx.iter().take(file_paths.len()).all(|_| { true });
    println!("{} {:?}", "Total obfuscation time:".green(), now.elapsed());
}

fn extract_file_paths_recursively(input: &String, output: &String) -> Vec<(PathBuf, PathBuf)> {
    WalkDir::new(input)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|de| {
            let is_file = de
                .metadata()
                .map_or(false, |md| md.is_file());

            let is_not_hidden = de
                .file_name()
                .to_str()
                .map_or(false, |s| !s.starts_with("."));

            is_file && is_not_hidden
        })
        .map(|de| {
            let de_relative_path = de
                .path()
                .to_str()
                .unwrap()
                .trim_start_matches(input)
                .trim_start_matches("/");

            let result_path_str = format!("{}/{}", output.trim_end_matches("/"), de_relative_path);
            let result_path = Path::new(&result_path_str);

            fs::create_dir_all(result_path.parent().unwrap()).unwrap();

            (de.path().to_path_buf(), result_path.to_path_buf())
        })
        .collect::<Vec<(PathBuf, PathBuf)>>()
}

fn replacer(raw: &str) -> String {
    format!("{}{}{}", MARKER, murmur3::hash128_with_seed(&raw, SEED), MARKER)
}