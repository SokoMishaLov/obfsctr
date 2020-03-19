use std::{
    fs,
    fs::File,
    fs::metadata,
    io,
    io::{Error, ErrorKind},
    path::Path,
};
use std::net::Shutdown::Read;

use clap::Clap;
use regex::{
    Regex,
    RegexSet,
};

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

fn extract_file_paths(path: &Path) -> io::Result<Vec<File>> {
    let md = metadata(&path)?;
    if md.is_dir() {
        let entries: Vec<File> = fs::read_dir(path)?
            .map(|res| res.map(|e| File::open(e.path().as_path())).unwrap())
            .collect::<Result<Vec<_>, io::Error>>()?;

        Ok(entries)
    } else if md.is_file() {
        let file = File::open(path)?;
        Ok(vec![file])
    } else {
        Err(Error::new(ErrorKind::NotFound, "File or directory does not exist!"))
    }
}

fn replacer(raw: String) -> String {
    String::from("kek")
}

fn main() {
    // let opts: Opts = Opts::parse();

    let opts = Opts {
        input: "/Users/mihailsokolov/Desktop/SMA/IdeaProjects/obfsctr/examples/".to_string(),
        output: "/Users/mihailsokolov/Desktop/SMA/IdeaProjects/obfsctr/examples/".to_string(),
        regex: r"(?:^|\W)and(?:$|\W)".to_string(),
        threads: 4,
    };

    let files = extract_file_paths(Path::new(&opts.input)).unwrap();

    for mut file in files {
        println!("Obfuscating {:?}", file);

        let r = Regex::new(opts.regex.as_str()).unwrap();

        file.obfuscate_by_regex(&r, replacer)
    }
}
