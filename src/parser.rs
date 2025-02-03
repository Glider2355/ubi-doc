use std::path::Path;

pub mod file_io;
pub mod get_ubiquitous;
pub mod language;
pub mod php;
pub mod ubiquitous;

pub fn get_comment(path: &Path) -> Vec<String> {
    let ubiquitous_list = get_ubiquitous::get_ubiquitous(path);

    ubiquitous_list
        .into_iter()
        .filter_map(|u| u.ubiquitous)
        .collect()
}
