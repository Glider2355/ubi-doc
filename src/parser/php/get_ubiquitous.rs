use tree_sitter::Parser;

use crate::parser::ubiquitous::Ubiquitous;

use super::{
    extract_ubiquitous::extract_ubiquitous, ubiquitous_node_collect::ubiquitous_node_collect,
};

pub fn get_ubiquitous(code: &str) -> Vec<Ubiquitous> {
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

    // ノードを再帰的に巡回しコメントを収集
    let comments = ubiquitous_node_collect(root_node, source_code);

    // コメント文字列をクリーニング
    extract_ubiquitous(comments)
}
