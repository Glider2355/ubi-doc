use tree_sitter::Parser;

/// 指定された PHP コードからすべてのコメントを取得する関数
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

    comments
}

/// 再帰関数でノードを巡回し、comment ノードがあればベクタに追加
fn visit_node(node: tree_sitter::Node, source_code: &[u8], comments: &mut Vec<String>) {
    // 自分自身が "comment" ノードの場合、コメントを取得して追加
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
    fn test_get_comments() {
        let code = r#"
<?php
    // 1行目のコメント
    echo "Hello, World!";
    /* ブロックコメント */
    // 2行目のコメント
?>
"#;

        let comments = get_comments(code);

        // 期待されるコメント一覧
        let expected = vec![
            "// 1行目のコメント",
            "/* ブロックコメント */",
            "// 2行目のコメント",
        ];

        assert_eq!(comments, expected, "取得したコメントが期待値と異なります");
    }
}
