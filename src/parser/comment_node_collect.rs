/// 再帰関数でノードを巡回し、comment ノードがあればベクタに追加
pub fn comment_node_collect(node: tree_sitter::Node, source_code: &[u8], comments: &mut Vec<String>) {
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