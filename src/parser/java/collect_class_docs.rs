use tree_sitter::Node;

#[derive(Debug)]
pub struct ClassDoc {
    pub class_name: String,
    pub doc_comment: String,
    pub doc_comment_line: Option<usize>,
}

pub fn collect_class_docs(node: Node, source_code: &[u8]) -> Vec<ClassDoc> {
    let mut results = Vec::new();

    // Java の class_declaration ノードを検出
    if node.kind() == "class_declaration" {
        if let Some(id_node) = find_class_identifier_top_level(node) {
            if let Ok(class_name) = id_node.utf8_text(source_code) {
                // クラス直前の JavaDoc/コメントを探す
                if let Some((doc_comment, doc_comment_line)) =
                    find_preceding_doc_comment_java(node, source_code)
                {
                    results.push(ClassDoc {
                        class_name: class_name.to_string(),
                        doc_comment,
                        doc_comment_line: Some(doc_comment_line),
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

/// class_declaration ノードの「直下の子」を順に確認し、
/// 最初に見つかった 'identifier' を返す。
fn find_class_identifier_top_level(class_decl: Node) -> Option<Node> {
    for i in 0..class_decl.child_count() {
        if let Some(child) = class_decl.child(i) {
            let kind = child.kind();
            // "class" キーワードのすぐあとに現れるクラス名は、
            // "identifier" というノード種別になっていることが多い。
            if kind == "identifier" {
                return Some(child);
            }
        }
    }
    None
}

/// JavaDoc (/** ... */) を探す処理
fn find_preceding_doc_comment_java(node: Node, source_code: &[u8]) -> Option<(String, usize)> {
    let mut current = node.prev_sibling();
    while let Some(prev) = current {
        let kind = prev.kind();
        // tree-sitter-java では "line_comment" / "block_comment" などがあり得る
        if kind == "comment" || kind == "line_comment" || kind == "block_comment" {
            if let Ok(comment_text) = prev.utf8_text(source_code) {
                // ここでは簡易的に "/**" で始まるかどうかでJavaDocと判断
                if comment_text.trim_start().starts_with("/**") {
                    let line = prev.start_position().row + 1;
                    return Some((comment_text.to_string(), line));
                }
            }
        }
        if prev.is_named()
            && kind != "comment"
            && kind != "line_comment"
            && kind != "block_comment"
        {
            break;
        }
        current = prev.prev_sibling();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;
    use tree_sitter_java;

    #[test]
    fn test_collect_class_docs_with_javadoc() {
        let code = r#"
        /**
         * A sample doc comment
         */
        public class Foo {
        }
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_java::LANGUAGE;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());
        assert_eq!(docs.len(), 1, "クラスが1つだけ取得されるはず");

        let foo = &docs[0];
        assert_eq!(foo.class_name, "Foo");
        assert!(
            foo.doc_comment.contains("A sample doc comment"),
            "JavaDoc内の文字列が含まれること"
        );
        assert_eq!(
            foo.doc_comment_line,
            Some(2),
            "JavaDocが始まる行(2行目)が取得されるはず"
        );
    }

    #[test]
    fn test_collect_class_docs_without_javadoc() {
        let code = r#"
        // 通常コメントのみ
        public class Bar {
        }
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_java::LANGUAGE;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());
        // 通常コメントのみでJavaDocはないので結果は0件
        assert_eq!(docs.len(), 0, "JavaDocなしのクラスは検出されないはず");
    }

    #[test]
    fn test_collect_class_docs_multiple_classes() {
        let code = r#"
        /**
         * First JavaDoc
         */
        public class Alpha {
        }

        // これはJavaDocではない普通のコメント
        /**
         * Second JavaDoc
         */
        public class Beta {
        }
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_java::LANGUAGE;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());
        assert_eq!(docs.len(), 2, "2つのクラスが検出されるはず");

        let alpha = &docs[0];
        assert_eq!(alpha.class_name, "Alpha");
        assert!(
            alpha.doc_comment.contains("First JavaDoc"),
            "AlphaクラスのJavaDocに 'First JavaDoc' が含まれる"
        );

        let beta = &docs[1];
        assert_eq!(beta.class_name, "Beta");
        assert!(
            beta.doc_comment.contains("Second JavaDoc"),
            "BetaクラスのJavaDocに 'Second JavaDoc' が含まれる"
        );
    }

    #[test]
    fn test_collect_class_docs_interrupted_by_other_node() {
        let code = r#"
        public class NoDoc {
        }

        // ここに別のクラスがあるので、次のJavaDocがその前に割り込まれる
        /**
         * JavaDoc for Bar
         */
        public class Bar {}
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_java::LANGUAGE;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());

        // Bar クラス1件だけが取得される (NoDocはJavaDocなし)
        assert_eq!(docs.len(), 1, "JavaDoc付きの Bar クラスだけが検出される");
        assert_eq!(docs[0].class_name, "Bar");
        assert!(
            docs[0].doc_comment.contains("JavaDoc for Bar"),
            "BarクラスのJavaDocが取れていること"
        );
    }
}
