use tree_sitter::Node;

#[derive(Debug)]
pub struct ClassDoc {
    pub class_name: String,
    pub doc_comment: String,
    pub doc_comment_line: Option<usize>,
}

pub fn collect_class_docs(node: Node, source_code: &[u8]) -> Vec<ClassDoc> {
    let mut results = Vec::new();

    // Ruby の class ノードを検出
    if node.kind() == "class" {
        if let Some(id_node) = find_class_identifier_top_level(node) {
            if let Ok(class_name) = id_node.utf8_text(source_code) {
                // クラス直前のコメントを探す
                if let Some((doc_comment, doc_comment_line)) =
                    find_preceding_doc_comments_ruby(node, source_code)
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

/// class ノードの「直下の子」を順に確認し、
/// 最初に見つかった 'constant' を返す。
fn find_class_identifier_top_level(class_decl: Node) -> Option<Node> {
    for i in 0..class_decl.child_count() {
        if let Some(child) = class_decl.child(i) {
            let kind = child.kind();
            // "class" キーワードのすぐあとに現れるクラス名は、
            // "constant" というノード種別になっていることが多い。
            if kind == "constant" {
                return Some(child);
            }
        }
    }
    None
}

/// 直前に連続するコメント行をすべて取得し、改行区切りでまとめて返す。
/// 返すタプルは (コメント全体, 最初(一番上)のコメント行番号)。
fn find_preceding_doc_comments_ruby(node: Node, source_code: &[u8]) -> Option<(String, usize)> {
    let mut comment_lines_reversed = Vec::new();
    // ここで型注釈を追加する
    let mut earliest_line: Option<usize> = None;

    let mut current = node.prev_sibling();

    while let Some(prev_node) = current {
        let kind = prev_node.kind();
        if kind == "comment" {
            if let Ok(comment_text) = prev_node.utf8_text(source_code) {
                let line = prev_node.start_position().row + 1;

                comment_lines_reversed.push(comment_text.to_string());

                // min() を使って最小行を記憶
                earliest_line = match earliest_line {
                    Some(current_min) => Some(current_min.min(line)),
                    None => Some(line),
                };
            }
        } else if prev_node.is_named() {
            // コメント以外の named ノードがあれば打ち切る
            break;
        }
        current = prev_node.prev_sibling();
    }

    if comment_lines_reversed.is_empty() {
        None
    } else {
        comment_lines_reversed.reverse();
        let all_comments = comment_lines_reversed.join("\n");

        // earliest_line は必ず Some に入っているので unwrap_or_default() でもよい
        Some((all_comments, earliest_line.unwrap_or_default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_collect_class_docs_with_comment() {
        let code = r#"
        # A sample doc comment
        class Foo
        end
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_ruby::LANGUAGE;

        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());
        assert_eq!(docs.len(), 1, "クラスが1つだけ取得されるはず");

        let foo = &docs[0];
        assert_eq!(foo.class_name, "Foo");
        assert!(
            foo.doc_comment.contains("A sample doc comment"),
            "コメント内の文字列が含まれること"
        );
        assert_eq!(
            foo.doc_comment_line,
            Some(2),
            "コメントが始まる行(2行目)が取得されるはず"
        );
    }

    #[test]
    fn test_collect_class_docs_multiple_classes() {
        let code = r#"
        # First doc comment
        class Alpha
        end

        # first comment
        # second comment
        class Beta
        end
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_ruby::LANGUAGE;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());
        assert_eq!(docs.len(), 2, "2つのクラスが検出されるはず");

        let alpha = &docs[0];
        assert_eq!(alpha.class_name, "Alpha");
        assert!(
            alpha.doc_comment.contains("First doc comment"),
            "Alphaクラスのコメントに 'First doc comment' が含まれる"
        );

        let beta = &docs[1];
        assert_eq!(beta.class_name, "Beta");
        assert!(
            beta.doc_comment.contains("first comment"),
            "Betaクラスのコメントに 'first comment' が含まれる"
        );
        assert!(
            beta.doc_comment.contains("second comment"),
            "Betaクラスのコメントに 'second comment' が含まれる"
        );
    }

    #[test]
    fn test_collect_class_docs_interrupted_by_other_node() {
        let code = r#"
        class NoDoc
        end

        # ここに別のクラスがあるので、次のコメントがその前に割り込まれる
        # Doc for Bar
        class Bar
        end
        "#;

        let mut parser = Parser::new();
        let language = tree_sitter_ruby::LANGUAGE;
        parser.set_language(&language.into()).unwrap();

        let tree = parser.parse(code, None).expect("Failed to parse code");
        let root_node = tree.root_node();

        let docs = collect_class_docs(root_node, code.as_bytes());

        // Bar クラス1件だけが取得される (NoDocはコメントなし)
        assert_eq!(docs.len(), 1, "コメント付きの Bar クラスだけが検出される");
        assert_eq!(docs[0].class_name, "Bar");
        assert!(
            docs[0].doc_comment.contains("Doc for Bar"),
            "Barクラスのコメントが取れていること"
        );
    }
}
