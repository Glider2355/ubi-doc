use tree_sitter::Parser;
use super::{comment_node_collect::comment_node_collect, extract_ubiquitous::extract_ubiquitous};

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
    extract_ubiquitous(comments)
}
