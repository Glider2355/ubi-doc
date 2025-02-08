use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub struct GenerateHtmlParam {
    pub class_name: String,
    pub ubiquitous: String,
    pub context: String,
    pub description: String,
    pub file_path: String,
    pub line_number: usize,
}

pub fn generate_html(ubiquitous_list: Vec<GenerateHtmlParam>, output_path: &Path) {
    // 出力先ディレクトリが存在しない場合は作成する
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let repo = std::env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "owner/repo".to_string());
    let branch = std::env::var("GITHUB_REF_NAME").unwrap_or_else(|_| "main".to_string());

    let mut html = String::new();
    html.push_str("<html>\n");
    html.push_str("  <head>\n");
    html.push_str("    <title>Ubiquitous Language</title>\n");
    html.push_str("  </head>\n");
    html.push_str("  <body>\n");
    html.push_str("    <h1>Ubiquitous Language</h1>\n");
    html.push_str("    <table border='1'>\n");
    html.push_str("      <tr><th>Ubiquitous</th><th>Class Name</th><th>Context</th><th>Description</th><th>URL</th></tr>\n");

    for ubiquitous in ubiquitous_list {
        let file_path_link = format!(
            "https://github.com/{}/blob/{}/{}#L{}",
            repo, branch, ubiquitous.file_path, ubiquitous.line_number
        );

        html.push_str("      <tr>\n");
        html.push_str(&format!("        <td>{}</td>\n", ubiquitous.ubiquitous));
        html.push_str(&format!("        <td>{}</td>\n", ubiquitous.class_name));
        html.push_str(&format!("        <td>{}</td>\n", ubiquitous.context));
        html.push_str(&format!("        <td>{}</td>\n", ubiquitous.description));
        html.push_str(&format!(
            "        <td><a href=\"{url}\">{file}:{line}</a></td>\n",
            url = file_path_link,
            file = ubiquitous.file_path,
            line = ubiquitous.line_number
        ));
        html.push_str("      </tr>\n");
    }

    html.push_str("    </table>\n");
    html.push_str("  </body>\n");
    html.push_str("</html>\n");

    let mut file = File::create(output_path).unwrap();
    file.write_all(html.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_generate_html() {
        // テスト用のデータを用意
        let test_params = vec![
            GenerateHtmlParam {
                class_name: "User".to_string(),
                ubiquitous: "ユビキタス".to_string(),
                context: "ユーザー".to_string(),
                description: "ユーザー情報".to_string(),
                file_path: "src/user.rs".to_string(),
                line_number: 10,
            },
            GenerateHtmlParam {
                class_name: "Item".to_string(),
                ubiquitous: "ユビキタス".to_string(),
                context: "アイテム".to_string(),
                description: "アイテム情報".to_string(),
                file_path: "src/item.rs".to_string(),
                line_number: 20,
            },
        ];

        // テストで利用する出力ファイルのパス
        let output_path = Path::new("tests/fixtures/test_output.html");

        // 出力先ディレクトリが存在しない場合は作成
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // HTML ファイル生成関数を実行
        generate_html(test_params, output_path);

        // 生成された HTML ファイルを読み込み
        let html_content = fs::read_to_string(output_path).unwrap();

        // HTML の各要素が正しく出力されているか検証
        assert!(html_content.contains("<html>"));
        assert!(html_content.contains("<title>Ubiquitous Language</title>"));
        assert!(html_content.contains("<h1>Ubiquitous Language</h1>"));
        assert!(html_content.contains("<table border='1'>"));

        // テーブルヘッダ: File Path → URL に変更されたので、ここも修正
        assert!(html_content.contains("<tr><th>Ubiquitous</th><th>Class Name</th><th>Context</th><th>Description</th><th>URL</th></tr>"));

        // 各フィールド
        assert!(html_content.contains("<td>ユビキタス</td>"));
        assert!(html_content.contains("<td>User</td>"));
        assert!(html_content.contains("<td>ユーザー</td>"));
        assert!(html_content.contains("<td>ユーザー情報</td>"));
        assert!(html_content.contains("<td>アイテム</td>"));
        assert!(html_content.contains("<td>アイテム情報</td>"));

        // リンクとして表示されているか。フォールバックでは "owner/repo" / "main" が使われる
        // user.rs (行10)
        assert!(html_content.contains("<a href=\"https://github.com/owner/repo/blob/main/src/user.rs#L10\">src/user.rs:10</a>"));
        // item.rs (行20)
        assert!(html_content.contains("<a href=\"https://github.com/owner/repo/blob/main/src/item.rs#L20\">src/item.rs:20</a>"));

        // テスト終了後にファイルを削除
        fs::remove_file(output_path).unwrap();
    }
}
