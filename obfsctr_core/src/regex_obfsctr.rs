use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use regex::{
    self,
    Captures,
    Regex,
    RegexSet,
};

pub trait RegexObfuscator {
    fn obfuscate(&self, dest: &mut Self, re_set: &RegexSet, replacer: fn(&str) -> String);
}

impl RegexObfuscator for String {
    fn obfuscate(&self, dest: &mut Self, re_set: &RegexSet, replacer: fn(&str) -> String) {
        *dest = (re_set.patterns() as &[String])
            .iter()
            .filter_map(|p| Regex::new(p.as_str()).ok())
            .fold(self.clone(), |accum, re| {
                re
                    .replace_all(accum.as_str(), |caps: &Captures| {
                        let val = caps.get(0).unwrap().as_str();
                        replacer(val).to_string()
                    })
                    .to_string()
            });
    }
}

impl RegexObfuscator for File {
    fn obfuscate(&self, dest: &mut Self, re_set: &RegexSet, replacer: fn(&str) -> String) {
        let mut buf_reader = BufReader::new(self);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        let mut obfuscated_content = String::new();
        content.obfuscate(&mut obfuscated_content, re_set, replacer);

        let mut buf_writer = BufWriter::new(dest);
        buf_writer.write_all(obfuscated_content.as_bytes()).unwrap();
    }
}

impl RegexObfuscator for &Path {
    fn obfuscate(&self, dest: &mut Self, re_set: &RegexSet, replacer: fn(&str) -> String) {
        let file_to_read = fs::OpenOptions::new().read(true).open(*self).unwrap();
        let mut file_to_write = fs::OpenOptions::new().write(true).create(true).open(*dest).unwrap();
        file_to_read.obfuscate(&mut file_to_write, re_set, replacer)
    }
}

impl RegexObfuscator for PathBuf {
    fn obfuscate(&self, dest: &mut Self, re_set: &RegexSet, replacer: fn(&str) -> String) {
        self.as_path().obfuscate(&mut dest.as_path(), re_set, replacer)
    }
}