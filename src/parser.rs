pub mod comment_paser;
pub mod extract_ubiquitous;
pub mod ubiquitous;
pub mod php;

pub fn get_comment(code: &str) -> Vec<String> {
    let ubiquitous_list = comment_paser::get_comments(code);

    ubiquitous_list
        .into_iter()
        .filter_map(|u| u.ubiquitous)
        .collect()
}
