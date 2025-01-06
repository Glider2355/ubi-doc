mod extract_comment;

pub fn extract_comments(comments: Vec<String>)-> Vec<String> {
    extract_comment::extract_comments(comments)
}