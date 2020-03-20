use std::{fs, fs::metadata, io, io::{Error, ErrorKind}, path::Path};
use std::path::PathBuf;

use clap::Clap;
use regex::Regex;

use obfsctr_core::regex_obfsctr::Obfuscator;

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
    threads: u8,
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
    raw.to_uppercase()
}

fn main() {
    let opts: Opts = Opts::parse();

    // let opts = Opts {
    //     input: "/Users/mihailsokolov/Desktop/SMA/IdeaProjects/obfsctr/examples/".to_string(),
    //     regex: r"and".to_string(),
    //     threads: 4,
    // };

    let file_paths = extract_file_paths(opts.input.as_str()).unwrap();

    for path in file_paths {
        let r = Regex::new(opts.regex.as_str()).unwrap();
        path.as_path().obfuscate_by_regex(&r, replacer);
    }
}
