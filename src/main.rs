use parser::get_comment;

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
