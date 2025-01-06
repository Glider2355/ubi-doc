mod comment_paser;
mod converter;

pub fn get_comment(code: &str) -> Vec<String> {
    comment_paser::get_comments(code)
}