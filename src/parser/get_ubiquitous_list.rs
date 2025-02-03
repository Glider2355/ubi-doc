use std::path::Path;

use crate::parser::{
    file_io::read_sources_from_dir::read_sources_from_dir, php::get_ubiquitous::get_ubiquitous,
};

use super::ubiquitous::Ubiquitous;

/// 拡張子を表す列挙型
#[derive(Debug)]
pub enum FileType {
    Php,
    Kotlin,
    Other(String), // 未知の拡張子など
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "php" => FileType::Php,
            "kt" => FileType::Kotlin,
            other => FileType::Other(other.to_string()),
        }
    }
}

pub fn get_ubiquitous_list(path: &Path) -> Vec<Ubiquitous> {
    let code_files = match read_sources_from_dir(path) {
        Ok(files) => files,
        Err(_) => return vec![],
    };

    // すべてのファイルを処理するので、ループして結果をまとめる
    let mut all_results = Vec::new();

    for code_file in code_files {
        match FileType::from_extension(&code_file.extension) {
            FileType::Php => {
                let ubiquitous = get_ubiquitous(&code_file.code);
                all_results.extend(ubiquitous);
            }
            FileType::Kotlin => {
                // Kotlin ファイルへの処理
            }
            FileType::Other(ext) => {
                // 未知の拡張子のファイルへの処理
                eprintln!("Unknown file extension: {}", ext);
            }
        }
    }
    all_results
}
