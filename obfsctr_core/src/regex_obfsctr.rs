use std::{
    fs,
    io::{BufReader, BufWriter},
    io::Read,
    io::Write,
    path::Path,
};

use regex::{self, Captures, Regex};

pub trait RegexObfuscator {
    fn obfuscate(&self, regex: &Regex, replacer: fn(&str) -> String) -> Self;
}

impl RegexObfuscator for String {
    fn obfuscate(&self, regex: &Regex, replacer: fn(&str) -> String) -> Self {
        let obfuscated = regex.replace_all(self, |caps: &Captures| {
            let val = caps.get(0).unwrap().as_str();
            replacer(val).to_string()
        }).to_string();
        obfuscated
    }
}

impl RegexObfuscator for &Path {
    fn obfuscate(&self, regex: &Regex, replacer: fn(&str) -> String) -> Self {
        let file_to_read = fs::OpenOptions::new().read(true).open(*self).unwrap();
        let mut buf_reader = BufReader::new(file_to_read);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        let obfuscated = content.obfuscate(regex, replacer);

        let file_to_write = fs::OpenOptions::new().write(true).open(*self).unwrap();
        let mut buf_writer = BufWriter::new(file_to_write);
        buf_writer.write_all(obfuscated.as_bytes()).unwrap();

        *self
    }
}