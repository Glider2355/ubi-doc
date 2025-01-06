use tree_sitter::Parser;
use super::converter::extract_comments;
use super::comment_node_collect::comment_node_collect;

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
    comment_node_collect(root_node, source_code, &mut comments);

    // コメント文字列をクリーニング
    extract_comments(comments)
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