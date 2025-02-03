use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// ファイルのコードと拡張子を保持する構造体
#[derive(Debug)]
pub struct CodeFile {
    pub code: String,
    pub extension: String,
}

/// ディレクトリ配下のソースコードを再帰的に走査して、CodeFile のリストを返す
pub fn read_sources_from_dir(dir_path: &Path) -> Result<Vec<CodeFile>, Box<dyn Error>> {
    let mut result = Vec::new();

    for entry in WalkDir::new(dir_path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_string();
                let code = fs::read_to_string(path)?;
                result.push(CodeFile {
                    code,
                    extension: ext_str,
                });
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_sources_from_dir() -> Result<(), Box<dyn Error>> {
        // 一時ディレクトリを作成
        let dir = tempdir()?;

        // ダミーファイルを1つ作成 (.rs ファイル)
        let file_path = dir.path().join("hello.rs");
        let mut file = File::create(&file_path)?;
        writeln!(file, "fn main() {{ println!(\"Hello, world!\"); }}")?;

        // テスト対象関数を呼び出し
        let results = read_sources_from_dir(dir.path())?;
        // 期待値: ファイルは1つだけのはず
        assert_eq!(results.len(), 1);

        let code_file = &results[0];
        assert_eq!(code_file.extension, "rs");
        assert!(code_file.code.contains("println!(\"Hello, world!\");"));

        Ok(())
    }

    #[test]
    fn test_read_sources_no_files() -> Result<(), Box<dyn Error>> {
        // 空の一時ディレクトリを作成
        let dir = tempdir()?;
        // テスト対象関数を呼び出し
        let results = read_sources_from_dir(dir.path())?;
        // 期待値: ファイルがないので結果は空のはず
        assert_eq!(results.len(), 0);

        Ok(())
    }

    #[test]
    fn test_read_sources_multiple_extensions() -> Result<(), Box<dyn Error>> {
        // 一時ディレクトリを作成
        let dir = tempdir()?;

        // hello.rs
        let file_path_rs = dir.path().join("hello.rs");
        let mut file_rs = File::create(&file_path_rs)?;
        writeln!(file_rs, "fn main() {{ println!(\"Hello, Rust!\"); }}")?;

        // hello.txt
        let file_path_txt = dir.path().join("hello.txt");
        let mut file_txt = File::create(&file_path_txt)?;
        writeln!(file_txt, "Hello, Text!")?;

        // テスト対象関数を呼び出し
        let results = read_sources_from_dir(dir.path())?;

        // 期待値: rsとtxtの2つ
        assert_eq!(results.len(), 2);

        let mut has_rs = false;
        let mut has_txt = false;

        for code_file in results {
            match code_file.extension.as_str() {
                "rs" => {
                    has_rs = true;
                    assert!(code_file.code.contains("Hello, Rust!"));
                }
                "txt" => {
                    has_txt = true;
                    assert!(code_file.code.contains("Hello, Text!"));
                }
                _ => panic!("想定外の拡張子が読み込まれました"),
            }
        }

        assert!(has_rs, ".rs ファイルが見つかりませんでした");
        assert!(has_txt, ".txt ファイルが見つかりませんでした");

        Ok(())
    }
}
