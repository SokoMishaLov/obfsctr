pub mod obfsctr {
    use regex::Regex;

    pub trait Obfuscator {
        fn obfuscate_by_regexes(&mut self, regexes: Vec<Regex>, replacer: fn(&str) -> str);
    }

    impl Obfuscator for str {
        fn obfuscate_by_regexes(&mut self, regexes: Vec<Regex>, replacer: fn(&str) -> str) {
            for r in regexes {
                r.capture_names().for_each(|it| {
                    println!("{}", replacer(it.unwrap()))
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {}