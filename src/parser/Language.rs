use std::path::Path;

// let language = parser::language::Language::from_path(&path);
// println!("Language: {:?}", language);

#[derive(Debug, PartialEq)]
pub enum Language {
    Php,
    Rust,
    Java,
    Unknown(String),
}

impl Language {
    /// ディレクトリパスやファイルパスを受け取り、拡張子を判別して `Language` を返す
    pub fn from_path<P: AsRef<Path>>(path: &P) -> Self {
        let p = path.as_ref();
        match p.extension().and_then(|ext| ext.to_str()) {
            Some("php") => Language::Php,
            Some("rs") => Language::Rust,
            Some("java") => Language::Java,
            Some(other) => Language::Unknown(other.to_string()),
            None => Language::Unknown("no_ext".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path() {
        let cases = [
            ("index.php", Language::Php),
            ("main.rs", Language::Rust),
            ("HelloWorld.java", Language::Java),
            ("src/unknown.xyz", Language::Unknown("xyz".into())),
            ("some/directory/", Language::Unknown("no_ext".into())),
            ("file_without_ext", Language::Unknown("no_ext".into())),
        ];

        for (input, expected) in cases {
            let got = Language::from_path(&input);
            assert_eq!(
                got, expected,
                "Language::from_path(\"{input}\") => {:?}, but expected {:?}",
                got, expected
            );
        }
    }
}
