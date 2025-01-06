mod comment_paser;
mod converter;
mod comment_node_collect;

pub fn get_comment(code: &str) -> Vec<String> {
    comment_paser::get_comments(code)
}