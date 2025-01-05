mod parser;

fn main() {
    let code = r#"
<?php
    // 1行目のコメント
    echo "Hello, World!";
    // 2行目のコメント
    /* ブロックコメント */
    /**
    * タグ無しのコメント
    * @タグ タグの説明
    */
?>
"#;

    // parser.rs の get_comments 関数を使ってコメントを取得
    let comments = parser::get_comments(code);

    println!("--- Comments ---");
    for comment in comments {
        println!("{}", comment);
    }
}
