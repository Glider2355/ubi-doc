pub fn extract_ubiquitous(comments: Vec<String>) -> Vec<String> {
    // get_ubiquitous で抽出して、空文字列は除外する
    comments
        .into_iter()
        .map(get_ubiquitous)    // 各要素で @ubiquitous の内容を抽出
        .filter(|s| !s.is_empty()) // 空文字列なら取り除く
        .collect()
}

fn get_ubiquitous(comment: String) -> String {
    let comment = comment.trim();

    for line in comment.lines() {
        let line = line.trim();
        if let Some(pos) = line.find("@ubiquitous") {
            let tag_len = "@ubiquitous".len();
            if pos + tag_len <= line.len() {
                // @ubiquitous の直後の文字列を返す
                return line[pos + tag_len..].trim().to_string();
            }
        }
    }
    // 見つからなければ空文字列
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_comment_get() {
        let input = String::from("// @ubiquitous ubiquitous langage");
        let expected = "ubiquitous langage";
        assert_eq!(get_ubiquitous(input), expected);
    }

    #[test]
    fn test_doc_block_comment_multi_line_get() {
        let input = String::from(
r#"/**
 * aaaa
 * @ubiquitous ubiquitous langage
 * bbbb
 * @context context
 */"#
        );
        let expected = "ubiquitous langage";
        assert_eq!(get_ubiquitous(input), expected);
    }
}
