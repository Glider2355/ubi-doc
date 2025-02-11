use super::ubiquitous_rows::UbiquitousRows;
use std::path::PathBuf;
use tera::{Context, Tera};

pub fn render_html(rows: UbiquitousRows) -> String {
    // コンパイル時に設定されたリポジトリのルートパスを取得
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut templates_path = PathBuf::from(manifest_dir);
    templates_path.push("src/outputs/html/templates/*.html");

    let tera = Tera::new(templates_path.to_str().unwrap())
        .expect("Failed to init Tera with templates/*.html");

    let mut context = Context::new();
    context.insert("items", &rows.rows);

    tera.render("ubiquitous.html", &context)
        .expect("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::{render_html, UbiquitousRows};
    use crate::outputs::html::ubiquitous_row::UbiquitousRow;

    #[test]
    fn test_render_html() {
        // テスト用に2行分のデータを作成
        let row1 = UbiquitousRow::new()
            .set_class_name("SampleClass".to_string())
            .set_ubiquitous("ubiquitous1".to_string())
            .set_context("Context1".to_string())
            .set_description("some description".to_string())
            .set_github_url(
                "owner/repo".to_string(),
                "main".to_string(),
                "src/file.rs".to_string(),
                10,
            );

        let row2 = UbiquitousRow::new()
            .set_class_name("AnotherClass".to_string())
            .set_ubiquitous("ubiquitous2".to_string())
            .set_context("Context2".to_string())
            .set_description("another description".to_string())
            .set_github_url(
                "owner/repo".to_string(),
                "dev".to_string(),
                "src/another.rs".to_string(),
                20,
            );

        let rows = UbiquitousRows {
            rows: vec![row1, row2],
        };

        // render_htmlを呼び出し、返ってきたHTML文字列を検証
        let output = render_html(rows);

        // 基本的なタグが含まれているか
        assert!(output.contains("<html>"));
        assert!(output.contains("<h1>Ubiquitous Language</h1>"));
        assert!(output.contains("<table"));
        assert!(output.contains("</table>"));

        // row1 のデータがHTMLに含まれているか
        assert!(output.contains("SampleClass"));
        assert!(output.contains("ubiquitous1"));
        assert!(output.contains("Context1"));
        assert!(output.contains("some description"));
        assert!(
            output.contains(r#"href="https://github.com/owner/repo/blob/main/src/file.rs#L10""#)
        );

        // row2 のデータ
        assert!(output.contains("AnotherClass"));
        assert!(output.contains("ubiquitous2"));
        assert!(output.contains("Context2"));
        assert!(output.contains("another description"));
        assert!(
            output.contains(r#"href="https://github.com/owner/repo/blob/dev/src/another.rs#L20""#)
        );
    }
}
