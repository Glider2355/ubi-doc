pub mod comment_paser;
pub mod comment_node_collect;
pub mod extract_ubiquitous;

pub fn get_comment(code: &str) -> Vec<String> {
    comment_paser::get_comments(code)
}