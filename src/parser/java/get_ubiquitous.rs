use tree_sitter::Parser;
use tree_sitter_java;

use crate::parser::ubiquitous::Ubiquitous;

use super::{
    collect_class_docs::collect_class_docs,
    extract_ubiquitous::{extract_ubiquitous, ExtractUbiquitousParam},
};

pub fn get_ubiquitous(code: &str, file_path: &str) -> Vec<Ubiquitous> {
    println!("Java parser called for file: {}", file_path);
    let source_code = code.as_bytes();

    // Java 言語用のパーサを作成
    let mut parser = Parser::new();
    // Java の言語定義を設定
    parser
        .set_language(&tree_sitter_java::language())
        .expect("Error loading Java grammar");

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    println!("Java parser: root_node kind = {}", root_node.kind());

    // ノードを再帰的に巡回しコメントを収集
    let class_docs = collect_class_docs(root_node, source_code);
    println!("Java parser: found {} class docs", class_docs.len());

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
    let result = extract_ubiquitous(params);
    println!("Java parser: extracted {} ubiquitous", result.len());
    result
}
