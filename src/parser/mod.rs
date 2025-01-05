use tree_sitter::Parser;

/// 指定された PHP コードからすべてのコメントを取得する関数
/// コメント文字列から `//` や `/* ... */`、`/** ... */` を除去し、
/// 各行先頭の `*` や余分な空白も可能な限り取り除きます。
pub fn get_comments(code: &str) -> Vec<String> {
    let source_code = code.as_bytes();

    // PHP 言語用のパーサを作成
    let mut parser = Parser::new();
    let language = tree_sitter_php::LANGUAGE_PHP;
    parser
        .set_language(&language.into())
        .expect("Error loading PHP parser");

    // コードをパースして構文木を生成
    let tree = parser.parse(code, None).unwrap();

    // ルートノードを取得
    let root_node = tree.root_node();

    // コメントを格納するベクタ
    let mut comments = Vec::new();

    // ノードを再帰的に巡回しコメントを収集
    visit_node(root_node, source_code, &mut comments);

    // コメント文字列をクリーニング
    comments.into_iter().map(cleanup_comment).collect()
}

/// 再帰関数でノードを巡回し、comment ノードがあればベクタに追加
fn visit_node(node: tree_sitter::Node, source_code: &[u8], comments: &mut Vec<String>) {
    // コメントノード
    if node.kind() == "comment" {
        if let Ok(comment_text) = node.utf8_text(source_code) {
            comments.push(comment_text.to_string());
        }
    }

    // 子ノードを再帰的に探索
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            visit_node(child, source_code, comments);
        }
    }
}

/// コメント文字列から `//`, `/* ... */`, `/** ... */` を取り除き、
/// DocBlock(`/**`)の場合は行頭の `*` も除去する
fn cleanup_comment(comment: String) -> String {
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
        // 各行の先頭にある空白や`*`をまとめて取り除く
        let lines: Vec<String> = content
            .lines()
            .map(|line| {
                // 先頭にある空白と`*`をまとめて取り除く
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comments_mixed() {
        let code = r#"
<?php
    // 行コメント1
    /* 通常ブロックコメント */
    /**
     * DocBlockコメント
     * @tag タグ説明
     */
    // 行コメント2
?>
"#;
        let comments = get_comments(code);

        // 期待値
        let expected = vec![
            "行コメント1",
            "通常ブロックコメント",
            "DocBlockコメント\n@tag タグ説明",
            "行コメント2"
        ];

        assert_eq!(comments, expected);
    }

    #[test]
    fn test_get_comments_no_comments() {
        let code = r#"
<?php
    echo "Hello";
?>
"#;
        let comments = get_comments(code);
        assert_eq!(comments.len(), 0);
    }

    #[test]
    fn test_docblock_multiline() {
        let code = r#"
<?php
        /**
 * タグ無しのコメント
 * @タグ タグの説明
 */
"#;
        let comments = get_comments(code);
        //assert_eq!(comments.len(), 1);

        // 改行で分割して内容をテスト
        assert_eq!(comments, vec!["タグ無しのコメント\n@タグ タグの説明"]);
    }
}
