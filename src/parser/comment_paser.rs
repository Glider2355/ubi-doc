use tree_sitter::Parser;
use super::converter::extract_comments;

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
    extract_comments(comments)
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