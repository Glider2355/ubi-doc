use rayon::prelude::*;
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
    Other(String),
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

    // parallel execution
    let all_results: Vec<Ubiquitous> = code_files
        .par_iter()
        .map(|code_file| {
            match FileType::from_extension(&code_file.extension) {
                FileType::Php => {
                    // PHP ファイルの場合の処理（Vec 型の結果を返す）
                    get_ubiquitous(&code_file.code, &code_file.file_path)
                }
                FileType::Kotlin => {
                    // Kotlin ファイルへの処理
                    Vec::new()
                }
                FileType::Other(_ext) => Vec::new(),
            }
        })
        .flatten()
        .collect();

    all_results
}
