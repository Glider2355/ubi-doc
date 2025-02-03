use tree_sitter::Parser;

use crate::parser::ubiquitous::Ubiquitous;

use super::{
    collect_class_docs::collect_class_docs,
    extract_ubiquitous::{extract_ubiquitous, ExtractUbiquitousParam},
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
    let class_docs = collect_class_docs(root_node, source_code);

    let params = class_docs
        .into_iter()
        .map(|doc| ExtractUbiquitousParam {
            class_name: doc.class_name,
            doc_comment: doc.doc_comment,
        })
        .collect();

    // コメント文字列をクリーニング
    extract_ubiquitous(params)
}
