use outputs::html::generate_html::{generate_html, GenerateHtmlParam};
use parser::get_ubiquitous_list::get_ubiquitous_list;
use std::path::Path;
mod outputs;
mod parser;

fn main() {
    // file_io
    // Step1: ディレクトリパスを受け取る
    // Step2: ファイルの拡張子から言語を判定
    // Step3: 言語パーサーを決定する
    // Step4: ファイルの中身を抽出する
    // parser
    // Step5: ファイルの中身をパーサーに渡す
    // Step6: ユビキタス言語を抽出する
    // output
    // Step7: HTMLに出力する

    let path = Path::new("tests/fixtures");
    // parser.rs の get_comments 関数を使ってコメントを取得
    let ubiquitous_list = get_ubiquitous_list(path);

    for ubiquitous in &ubiquitous_list {
        println!(
            "ubiquitous: {}",
            ubiquitous
                .ubiquitous
                .as_deref()
                .unwrap_or("ubiquitousがNone")
        );
        println!(
            "class_name: {}",
            ubiquitous
                .class_name
                .as_deref()
                .unwrap_or("class_nameがNone")
        );
        println!(
            "context: {}",
            ubiquitous.context.as_deref().unwrap_or("contextがNone")
        );
        println!(
            "description: {}",
            ubiquitous
                .description
                .as_deref()
                .unwrap_or("descriptionがNone")
        );
        println!(
            "file_path: {}",
            ubiquitous.file_path.as_deref().unwrap_or("file_pathがNone")
        );
        println!(
            "line_number: {}",
            ubiquitous
                .line_number
                .map(|n| n.to_string())
                .unwrap_or_else(|| "line_numberがNone".to_string())
        );
    }

    // output
    // HTMLに出力する
    let output_path = Path::new("tests/fixtures/output.html");

    // その後、iter() で参照から再びデータを利用できる
    let mapped_list: Vec<GenerateHtmlParam> = ubiquitous_list
        .into_iter()
        .map(|u| GenerateHtmlParam {
            class_name: u.class_name.clone().unwrap_or_default(),
            ubiquitous: u.ubiquitous.clone().unwrap_or_default(),
            context: u.context.clone().unwrap_or_default(),
            description: u.description.clone().unwrap_or_default(),
            file_path: u.file_path.clone().unwrap_or_default(),
            line_number: u.line_number.unwrap_or(0),
        })
        .collect();

    generate_html(mapped_list, output_path);
}
