pub mod obfsctr {
    use std::fs::File;

    use regex::Regex;

    pub trait Obfuscator {
        fn obfuscate_by_regexes(&mut self, regexes: Vec<Regex>, replacer: fn(String) -> String);
    }

    impl Obfuscator for str {
        fn obfuscate_by_regexes(&mut self, regexes: Vec<Regex>, replacer: fn(String) -> String) {
            for r in regexes {
                r.capture_names().for_each(|it| {
                    println!("{}", replacer(String::from(it.unwrap())))
                })
            }
        }
    }

    impl Obfuscator for File {
        fn obfuscate_by_regexes(&mut self, regexes: Vec<Regex>, replacer: fn(String) -> String) {
            println!("todo")
        }
    }
}

#[cfg(test)]
mod tests {}
