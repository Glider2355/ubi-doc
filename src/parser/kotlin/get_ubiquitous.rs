use tree_sitter::Parser;
use tree_sitter_kotlin as kotlin;

use crate::parser::ubiquitous::Ubiquitous;

use super::{
    collect_class_docs::collect_class_docs,
    extract_ubiquitous::{extract_ubiquitous, ExtractUbiquitousParam},
};

pub fn get_ubiquitous(code: &str, file_path: &str) -> Vec<Ubiquitous> {
    let source_code = code.as_bytes();

    // PHP 言語用のパーサを作成
    let mut parser = Parser::new();
    // Kotlin の言語定義を設定
    parser
        .set_language(&kotlin::language())
        .expect("Error loading Kotlin grammar");

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    // ノードを再帰的に巡回しコメントを収集
    let class_docs = collect_class_docs(root_node, source_code);

    let params = class_docs
        .into_iter()
        .map(|doc| ExtractUbiquitousParam {
            class_name: doc.class_name,
            doc_comment: doc.doc_comment,
            file_path: file_path.to_string(),
            line_number: doc.doc_comment_line.unwrap_or(0),
        })
        .collect();

    // コメント文字列をクリーニング
    extract_ubiquitous(params)
}
