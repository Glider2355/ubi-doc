use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum SupportedLanguage {
    Php,
    Unknown,
}

pub fn guess_language_by_extension<P: AsRef<Path>>(path: P) -> SupportedLanguage {
    let extension = path
        .as_ref()
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    match extension {
        "php" => SupportedLanguage::Php,
        _ => SupportedLanguage::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_language_by_extension_php() {
        let php_path = "tests/fixtures/sample.php";
        let lang = guess_language_by_extension(php_path);
        assert_eq!(lang, SupportedLanguage::Php);
    }

    #[test]
    fn test_guess_language_by_extension_unknown() {
        let unknown_path = "tests/fixtures/sample.unknown";
        let lang = guess_language_by_extension(unknown_path);
        assert_eq!(lang, SupportedLanguage::Unknown);
    }

    #[test]
    fn test_guess_language_no_extension() {
        let no_extension = "tests/fixtures/file";
        let lang = guess_language_by_extension(no_extension);
        assert_eq!(lang, SupportedLanguage::Unknown);
    }
}
