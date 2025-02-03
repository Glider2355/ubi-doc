use tree_sitter::Node;

/// (クラス名, Docコメント) を格納するための型
pub struct ClassDoc {
    pub class_name: String,
    pub doc_comment: String,
}

/// クラス宣言に付随している Docコメントを探し出し、(クラス名, Docコメント) のリストを返す
pub fn collect_class_docs(node: Node, source_code: &[u8]) -> Vec<ClassDoc> {
    let mut results = Vec::new();

    // 自身がクラス宣言ノードかどうか
    if node.kind() == "class_declaration" {
        // ツリーシッターPHPでは class_declaration の中に "name" というフィールドがあるのでそれを探す
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(class_name) = name_node.utf8_text(source_code) {
                // クラス直前にある DocBlock コメントを探す
                if let Some(doc_comment) = find_preceding_doc_comment(node, source_code) {
                    results.push(ClassDoc {
                        class_name: class_name.to_string(),
                        doc_comment,
                    });
                }
            }
        }
    }

    // 子ノードを再帰的に探索
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            results.extend(collect_class_docs(child, source_code));
        }
    }

    results
}

/// クラス宣言ノードの直前にある Docコメント(コメントノード)を探して返す
fn find_preceding_doc_comment(node: Node, source_code: &[u8]) -> Option<String> {
    // 兄弟ノードを逆方向にたどりながらコメントを探す
    let mut current = node.prev_sibling();
    while let Some(prev) = current {
        // "comment" ノードならチェックする
        if prev.kind() == "comment" {
            if let Ok(comment_text) = prev.utf8_text(source_code) {
                // 今回は「Docコメント = '/**' で始まるもの」と簡易的に判断
                // 必要に応じてもう少し厳密にチェック
                if comment_text.trim_start().starts_with("/**") {
                    return Some(comment_text.to_string());
                }
            }
        }
        // 何か別の named ノード（function_declaration など）が出てきたら、
        // それ以上さかのぼらず打ち切る例
        if prev.is_named() && prev.kind() != "comment" {
            break;
        }
        // さらに前をチェック
        current = prev.prev_sibling();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_collect_class_docs_simple() {
        let code = r#"
<?php
/**
 * Foo class doc
 */
class Foo {
}
"#;

        // 1. パーサーの用意 & PHP 言語を設定
        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser
            .set_language(&language.into())
            .expect("Error loading PHP parser");

        // 2. コードをパースして抽象構文木(AST)を取得
        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        // 3. テスト対象の関数を呼び出し
        let docs = collect_class_docs(root_node, code.as_bytes());

        // 4. 結果を検証
        // Foo クラスが1つだけ取得される
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].class_name, "Foo");
        assert!(docs[0].doc_comment.contains("Foo class doc"));
    }

    #[test]
    fn test_collect_class_docs_multiple() {
        let code = r#"
<?php
/**
 * Doc for Foo
 */
class Foo {
}

/**
 * Doc for Bar
 */
class Bar {}

/**
 * This is not a class
 */
function baz() {}
"#;

        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());

        // Foo と Bar の2クラスが取得される
        assert_eq!(docs.len(), 2);

        assert_eq!(docs[0].class_name, "Foo");
        assert!(docs[0].doc_comment.contains("Doc for Foo"));
        assert_eq!(docs[1].class_name, "Bar");
        assert!(docs[1].doc_comment.contains("Doc for Bar"));
    }

    #[test]
    fn test_collect_class_docs_no_doc() {
        let code = r#"
<?php
// これはクラスの上に DocBlock がない例
class NoDocClass {
}
"#;

        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());

        // DocBlock が付いていないので 0 件
        assert_eq!(docs.len(), 0);
    }

    #[test]
    fn test_collect_class_docs_docblock_and_comment() {
        let code = r#"
<?php
// 一般的なコメント (DocBlock ではない)
/// Another style comment (C# style, not recognized as doc)
/**
 * Actual DocBlock
 */
class WithDocBlock {}
"#;

        let mut parser = Parser::new();
        let language = tree_sitter_php::LANGUAGE_PHP;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).unwrap();
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());

        // WithDocBlock クラスが1件見つかるはず
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].class_name, "WithDocBlock");
        assert!(docs[0].doc_comment.contains("Actual DocBlock"));
    }
}
