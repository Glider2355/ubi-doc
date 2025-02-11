use clap::Parser;
use outputs::html::{
    generate_html::generate_html, ubiquitous_row::UbiquitousRow, ubiquitous_rows::UbiquitousRows,
};
use std::path::Path;

use parser::get_ubiquitous_list::get_ubiquitous_list;
mod outputs;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "ユビキタス言語表作成ツール", long_about = None)]
struct Args {
    /// 入力ディレクトリのパス
    #[arg(short, long, default_value = "sample")]
    input: String,

    /// 出力HTMLファイルのパス
    #[arg(short, long, default_value = "docs")]
    output: String,
}

fn main() {
    // コマンドライン引数のパース
    let args = Args::parse();

    // 入力ディレクトリのパスを取得
    let input_path = Path::new(&args.input);
    let ubiquitous_list = get_ubiquitous_list(input_path);

    // 出力ファイルのパスを取得
    let output_path = Path::new(&args.output);

    let repo: String =
        std::env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "Glider2355/ubi-doc".to_string());
    let branch: String = std::env::var("GITHUB_REF_NAME").unwrap_or_else(|_| "main".to_string());

    let mut ubiquitous_rows = UbiquitousRows::new();
    for ubiquitous in ubiquitous_list.iter() {
        let row = UbiquitousRow::new()
            .set_class_name(ubiquitous.class_name.clone().unwrap_or_default())
            .set_ubiquitous(ubiquitous.ubiquitous.clone())
            .set_context(ubiquitous.context.clone().unwrap_or_default())
            .set_description(ubiquitous.description.clone().unwrap_or_default())
            .set_github_url(
                repo.clone(),
                branch.clone(),
                ubiquitous.file_path.clone().unwrap_or_default(),
                ubiquitous.line_number.unwrap_or_default(),
            );
        ubiquitous_rows.add(row);
    }

    // HTMLファイルとして出力
    generate_html(ubiquitous_rows, output_path);
}
