use std::{
    borrow::Borrow,
    borrow::BorrowMut,
    convert::TryInto,
    fmt::Debug,
    fs,
    fs::File,
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    io::Read,
    io::Write,
    ops::Deref,
    path::Path,
};

use regex::{
    self,
    Regex,
};

pub trait Obfuscator {
    fn obfuscate_by_regex(&self, regex: &Regex, replacer: fn(&str) -> &str) -> Self;
}

impl Obfuscator for String {
    fn obfuscate_by_regex(&self, regex: &Regex, replacer: fn(&str) -> &str) -> Self {
        let obfuscated = regex.replace_all(self, replacer("")).to_string();
        obfuscated
    }
}

impl Obfuscator for &Path {
    fn obfuscate_by_regex(&self, regex: &Regex, replacer: fn(&str) -> &str) -> Self {
        let file_to_read = fs::OpenOptions::new().read(true).open(*self).unwrap();
        let mut buf_reader = BufReader::new(file_to_read);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();

        let obfuscated = content.obfuscate_by_regex(regex, replacer);
        println!("{}", obfuscated);


        let file_to_write = fs::OpenOptions::new().write(true).open(*self).unwrap();
        println!("{:?}", file_to_write);
        let mut buf_writer = BufWriter::new(file_to_write);
        buf_writer.write_all(obfuscated.as_bytes()).unwrap();

        *self
    }
}
