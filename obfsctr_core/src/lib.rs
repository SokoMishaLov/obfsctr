pub mod obfsctr {
    pub trait Obfuscator {
        fn obfuscate(content: &mut String);
    }
}

#[cfg(test)]
mod tests {}