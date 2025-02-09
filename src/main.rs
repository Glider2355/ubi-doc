use clap::Parser;
use outputs::html::{generate_html::generate_html, ubiquitous_row::UbiquitousRow};
use std::path::Path;

use parser::get_ubiquitous_list::get_ubiquitous_list;
mod outputs;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "ユビキタス言語表作成ツール", long_about = None)]
struct Args {
    /// 入力ディレクトリのパス
    #[arg(short, long, default_value = "tests/fixtures")]
    input: String,

    /// 出力HTMLファイルのパス
    #[arg(short, long, default_value = "tests/fixtures")]
    output: String,
}

fn main() {
    // コマンドライン引数のパース
    let args = Args::parse();

    // 入力ディレクトリのパスを取得
    let input_path = Path::new(&args.input);
    let ubiquitous_list = get_ubiquitous_list(input_path);

    // 取得した情報をターミナルに表示（デバッグ用）
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

    // 出力ファイルのパスを取得
    let output_path = Path::new(&args.output);

    let repo: String =
        std::env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "owner/repo".to_string());
    let branch: String = std::env::var("GITHUB_REF_NAME").unwrap_or_else(|_| "main".to_string());

    let ubiquitous_rows: Vec<UbiquitousRow> = ubiquitous_list
        .iter()
        .map(|ubiquitous| {
            UbiquitousRow::new()
                .set_class_name(ubiquitous.class_name.clone().unwrap_or_default())
                .set_ubiquitous(ubiquitous.ubiquitous.clone().unwrap_or_default())
                .set_context(ubiquitous.context.clone().unwrap_or_default())
                .set_description(ubiquitous.description.clone().unwrap_or_default())
                .set_github_url(
                    repo.clone(),
                    branch.clone(),
                    ubiquitous.file_path.clone().unwrap_or_default(),
                    ubiquitous.line_number.unwrap_or_default(),
                )
        })
        .collect();

    // HTMLファイルとして出力
    generate_html(ubiquitous_rows, output_path);
}
