
pub fn extract_comments(comments: Vec<String>)-> Vec<String> {
    comments.into_iter().map(extract_comment).collect()
}

/// コメント文字列から `//`, `/* ... */`, `/** ... */` を取り除き、
/// DocBlock(`/**`)の場合は行頭の `*` も除去する。
fn extract_comment(comment: String) -> String {
    let comment = comment.trim();

    // 1行コメント: `// ...`
    if comment.starts_with("//") {
        return comment
            .trim_start_matches("//")
            .trim()
            .to_string();
    }

    // DocBlockコメント: `/** ... */`
    if comment.starts_with("/**") && comment.ends_with("*/") {
        let content = comment
            .trim_start_matches("/**")
            .trim_end_matches("*/")
            .trim();
        // 各行の先頭にある空白や `*` をまとめて取り除く
        let lines: Vec<String> = content
            .lines()
            .map(|line| {
                let line = line.trim_start();
                let line = line.strip_prefix('*').unwrap_or(line).trim_start();
                line.to_string()
            })
            .collect();
        return lines.join("\n");
    }

    // 通常ブロックコメント: `/* ... */`
    if comment.starts_with("/*") && comment.ends_with("*/") {
        return comment
            .trim_start_matches("/*")
            .trim_end_matches("*/")
            .trim()
            .to_string();
    }

    // 上記以外（万が一）はそのまま返す
    comment.to_string()
}


// テストモジュール。
// プライベート関数でも同じファイル内(モジュール内)であればテスト可能。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_comment() {
        let input = String::from("// line comment");
        let expected = "line comment";
        assert_eq!(extract_comment(input), expected);
    }

    #[test]
    fn test_block_comment() {
        let input = String::from("/* block comment */");
        let expected = "block comment";
        assert_eq!(extract_comment(input), expected);
    }

    #[test]
    fn test_doc_block_comment_single_line() {
        let input = String::from("/** doc block */");
        let expected = "doc block";
        assert_eq!(extract_comment(input), expected);
    }

    #[test]
    fn test_doc_block_comment_multi_line() {
        let input = String::from(
r#"/**
 * これは複数行にわたる
 * DocBlockコメント
 */"#
        );
        let expected = "これは複数行にわたる\nDocBlockコメント";
        assert_eq!(extract_comment(input), expected);
    }

    #[test]
    fn test_non_comment() {
        let input = String::from("Hello, World!");
        let expected = "Hello, World!";
        assert_eq!(extract_comment(input), expected);
    }
}

