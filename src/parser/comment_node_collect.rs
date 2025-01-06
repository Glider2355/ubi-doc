use tree_sitter::Node;

pub fn comment_node_collect(node: Node, source_code: &[u8], comments: &mut Vec<String>) {
    // コメントノード
    if node.kind() == "comment" {
        if let Ok(comment_text) = node.utf8_text(source_code) {
            comments.push(comment_text.to_string());
        }
    }

    // 子ノードを再帰的に探索
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            comment_node_collect(child, source_code, comments);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;
    #[test]
    fn test_comment_node_collect_simple() {
        // テスト用のPHPコード (最小例)
        let code = r#"
<?php
// line comment
?>
"#;

        // 1. パーサーを用意し、PHP言語設定をセット
        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP; 
        parser
            .set_language(&language.into())
            .expect("Error loading PHP parser");

        // 2. コードをパースしてツリーを生成
        let tree = parser
            .parse(code, None)
            .expect("Failed to parse code");
        let root_node = tree.root_node();

        // 3. テスト対象の関数に渡す
        let mut comments = Vec::new();
        comment_node_collect(root_node, code.as_bytes(), &mut comments);

        // 4. 結果を検証
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0], "// line comment");
    }

    #[test]
    fn test_comment_node_collect_multiple() {
        // 複数コメントの例
        let code = r#"
<?php
// first comment
/* block comment */
/**
 * DocBlock comment
 */
?>
"#;
        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let mut comments = Vec::new();
        comment_node_collect(root_node, code.as_bytes(), &mut comments);

        assert_eq!(comments.len(), 3);
        assert_eq!(comments[0], "// first comment");
        assert_eq!(comments[1], "/* block comment */");
        assert_eq!(comments[2], "/**\n * DocBlock comment\n */");
    }
}
