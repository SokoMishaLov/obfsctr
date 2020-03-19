use std::{
    fs,
    fs::File,
    io::Read,
};
use std::borrow::Borrow;
use std::convert::TryInto;
use std::io::Write;
use std::ops::Deref;

use regex;
use regex::Regex;

pub trait Obfuscator {
    fn obfuscate_by_regex(&mut self, regex: &Regex, replacer: fn(String) -> String);
}

impl Obfuscator for String {
    fn obfuscate_by_regex(&mut self, regex: &Regex, replacer: fn(String) -> String) {
        *self = regex.replace_all(self, "").to_string();
    }
}

impl Obfuscator for File {
    fn obfuscate_by_regex(&mut self, regex: &Regex, replacer: fn(String) -> String) {
        let mut content = String::new();
        self.read_to_string(&mut content);
        content.obfuscate_by_regex(regex, replacer);
        self.write_all(content.as_bytes()).unwrap_or_else(|_| {})
    }
}
