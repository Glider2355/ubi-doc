use serde::Serialize;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use tera::{Context, Tera};

pub struct GenerateHtmlParam {
    pub class_name: String,
    pub ubiquitous: String,
    pub context: String,
    pub description: String,
    pub file_path: String,
    pub line_number: usize,
}

// テンプレートに渡す用の構造体
#[derive(Serialize)]
struct TplItem {
    class_name: String,
    ubiquitous: String,
    context: String,
    description: String,
    file_path: String,
    line_number: usize,
    url: String,
}

pub fn generate_html(
    ubiquitous_list: Vec<GenerateHtmlParam>,
    repo: &String,
    branch: &String,
    output_path: &Path,
) {
    // 出力先ディレクトリが存在しない場合は作成
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // 1. Teraインスタンスを作成し、templates/*.html を読み込む
    //    (templatesディレクトリ内のファイルが対象)
    let tera = Tera::new("src/outputs/html/templates/*.html")
        .expect("Failed to init Tera with templates/*.html");

    // 2. テンプレートに渡すデータを準備
    //    リンクなどは Rust 側で生成しておく
    let items: Vec<TplItem> = ubiquitous_list
        .into_iter()
        .map(|u| {
            let url_link = format!(
                "https://github.com/{}/blob/{}/{}#L{}",
                repo, branch, u.file_path, u.line_number
            );
            TplItem {
                class_name: u.class_name,
                ubiquitous: u.ubiquitous,
                context: u.context,
                description: u.description,
                file_path: u.file_path,
                line_number: u.line_number,
                url: url_link,
            }
        })
        .collect();

    // 3. Contextを使ってデータを挿入
    let mut context = Context::new();
    // テンプレートでは {% for item in items %} と書いているので "items" のキーを使う
    context.insert("items", &items);

    // 4. テンプレートファイル "ubiquitous.html" をレンダリング
    let rendered_html = tera
        .render("ubiquitous.html", &context)
        .expect("Failed to render template");

    // 5. レンダリング結果をファイルに書き込む
    let mut file = File::create(output_path).unwrap();
    file.write_all(rendered_html.as_bytes()).unwrap();
}

// --- テストは既存のままでOK ---
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
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let repo = "owner/repo".to_string();
        let branch = "main".to_string();

        // HTML ファイル生成関数を実行
        generate_html(test_params, &repo, &branch, output_path);

        // 生成された HTML ファイルを読み込み
        let html_content = fs::read_to_string(output_path).unwrap();

        // テストでHTML要素が含まれるかをチェック（従来のテストをそのまま実行）
        assert!(html_content.contains("<html>"));
        assert!(html_content.contains("<title>Ubiquitous Language</title>"));
        assert!(html_content.contains("<h1>Ubiquitous Language</h1>"));
        assert!(html_content.contains("<table border='1'>"));
        assert!(html_content.contains("<tr><th>Ubiquitous</th><th>Class Name</th><th>Context</th><th>Description</th><th>URL</th></tr>"));
        assert!(html_content.contains("<td>ユビキタス</td>"));
        assert!(html_content.contains("<td>User</td>"));
        assert!(html_content.contains("<td>ユーザー</td>"));
        assert!(html_content.contains("<td>ユーザー情報</td>"));
        assert!(html_content.contains("<td>アイテム</td>"));
        assert!(html_content.contains("<td>アイテム情報</td>"));
        assert!(html_content
            .contains(r#"<a href="https://github.com/owner/repo/blob/main/src/user.rs#L10"#));
        assert!(html_content.contains(r#"src/user.rs:10"#));
        assert!(html_content
            .contains(r#"<a href="https://github.com/owner/repo/blob/main/src/item.rs#L20"#));
        assert!(html_content.contains(r#"src/item.rs:20"#));

        // テスト終了後にファイルを削除
        fs::remove_file(output_path).unwrap();
    }
}
