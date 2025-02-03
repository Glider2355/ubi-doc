use std::path::Path;

use crate::parser::{
    file_io::read_sources_from_dir::read_sources_from_dir, php::get_comments::get_comments,
};

use super::ubiquitous::Ubiquitous;

pub fn get_ubiquitous(path: &Path) -> Vec<Ubiquitous> {
    let code_files = match read_sources_from_dir(path) {
        Ok(files) => files,
        Err(_) => return vec![],
    };

    // とりあえず最初のファイルだけ使いたいケース
    if let Some(code_file) = code_files.into_iter().next() {
        if code_file.extension == "php" {
            return get_comments(&code_file.code);
        }
    }

    vec![]
}
