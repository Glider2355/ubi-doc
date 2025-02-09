use clap::Parser;
use std::path::Path;

use outputs::html::generate_html::{generate_html, GenerateHtmlParam};
use parser::get_ubiquitous_list::get_ubiquitous_list;
mod outputs;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "ユビキタス言語表作成ツール", long_about = None)]
struct Args {
    /// 入力ディレクトリのパス
    #[arg(short, long, default_value = ".")]
    input: String,

    /// 出力HTMLファイルのパス
    #[arg(short, long, default_value = "tests/fixtures/output.html")]
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

    // HTML 出力用のパラメータに変換
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

    let repo: String =
        std::env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "owner/repo".to_string());
    let branch: String = std::env::var("GITHUB_REF_NAME").unwrap_or_else(|_| "main".to_string());

    // HTMLファイルとして出力
    generate_html(mapped_list, &repo, &branch, output_path);
}
