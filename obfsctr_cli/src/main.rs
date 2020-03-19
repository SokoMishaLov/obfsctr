use std::{
    fs::metadata,
    io,
    io::{Error, ErrorKind},
    path::Path,
};
use std::borrow::Borrow;
use std::ops::Deref;

use clap::Clap;
use regex::Regex;

use obfsctr_core::regex_obfsctr::Obfuscator;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "sokomishalov")]
struct Opts {
    /// Sets an input file or directory
    #[clap(short = "i", long = "input")]
    input: String,

    /// Sets an output file or directory (current directory by default)
    #[clap(short = "o", long = "output", default_value = ".")]
    output: String,

    /// Sets a regex for
    #[clap(short = "r", long = "regex", default_value = "")]
    regex: String,

    /// Sets a number of worker threads (4 by default)
    #[clap(short = "n", long = "n-threads", default_value = "4")]
    threads: u8,
}

fn extract_file_paths(input_path: &str) -> io::Result<Vec<&Path>> {
    let path = Path::new(input_path);
    let md = metadata(&path)?;
    Ok(vec![path])

    // if md.is_dir() {
    //     let entries: Vec<&Path> = fs::read_dir(path)?
    //         .map(|res| res.map(|e| e.path().as_path()).unwrap())
    //         .collect::<Result<Vec<&Path>, io::Error>>()?;
    //
    //     Ok(entries)
    // } else if md.is_file() {
    //     Ok(vec![path])
    // } else {
    //     Err(Error::new(ErrorKind::NotFound, "File or directory does not exist!"))
    // }
}

fn replacer(raw: &str) -> &str {
    "kek"
}

fn main() {
    // let opts: Opts = Opts::parse();

    let opts = Opts {
        input: "/Users/mihailsokolov/Desktop/SMA/IdeaProjects/obfsctr/examples/hamlet.txt".to_string(),
        output: "/Users/mihailsokolov/Desktop/SMA/IdeaProjects/obfsctr/examples/".to_string(),
        regex: r"and".to_string(),
        threads: 4,
    };

    let file_paths = extract_file_paths(opts.input.as_str()).unwrap();

    for mut file_path in file_paths {
        let r = Regex::new(opts.regex.as_str()).unwrap();
        file_path.obfuscate_by_regex(&r, replacer);
    }
}
