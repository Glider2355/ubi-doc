use parser::get_comment;

mod parser;
mod file_io;

fn main() {
    // 
    // Step1: ディレクトリパスを受け取る
    // Step2: ファイルの拡張子から言語を判定
    // Step3: 言語パーサーを決定する
    // Step4: ファイルの中身を抽出する
    // 
    // Step5: ファイルの中身をパーサーに渡す
    // Step6: ユビキタス言語を抽出する
    // Step7: HTMLに出力する


    let code = r#"
<?php
    // 1行目のコメント
    echo "Hello, World!";
    // 2行目のコメント
    /* ブロックコメント */
    /**
    * タグ無しのコメント
    * @ubiquitous ubiquitous langage
    */
?>
"#;

    // parser.rs の get_comments 関数を使ってコメントを取得
    let comments = get_comment(code);

    println!("--- Comments ---");
    for comment in comments {
        println!("{}", comment);
    }
}
