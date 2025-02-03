use std::path::Path;

use parser::get_comment;

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

    let path = Path::new("tests/fixtures/sample.php");
    // parser.rs の get_comments 関数を使ってコメントを取得
    let comments = get_comment(path);

    println!("--- Comments ---");
    for comment in comments {
        println!("{}", comment);
    }
}
