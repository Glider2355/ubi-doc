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
