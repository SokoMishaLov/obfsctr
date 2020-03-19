pub mod obfsctr {
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
    use regex::{
        Regex,
        RegexSet,
    };

    // raw - obfuscated
    type Replacement = (String, String);

    pub trait Obfuscator {
        fn obfuscate_by_regexes(&mut self, regexes: &RegexSet, replacer: fn(String) -> String);
    }

    impl Obfuscator for String {
        fn obfuscate_by_regexes(&mut self, regexes: &RegexSet, replacer: fn(String) -> String) {
            for r in regexes {
                r.capture_names().for_each(|it| {
                    let raw = String::from(it.unwrap());
                    let obfuscated = replacer(raw);

                    let res = self.replace(&raw, &obfuscated);
                });
                r.replace_all()
            }
        }
    }

    impl Obfuscator for File {
        fn obfuscate_by_regexes(&mut self, regexes: &RegexSet, replacer: fn(String) -> String) {
            let mut content = String::new();
            self.read_to_string(&mut content);
            content.obfuscate_by_regexes(regexes, replacer);
            self.write_all(content.as_bytes()).unwrap_or_else(|_| {})
        }
    }
}

#[cfg(test)]
mod tests {}
