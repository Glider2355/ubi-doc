use tera::{Context, Tera};

use super::ubiquitous_row::UbiquitousRow;

pub fn render_html(rows: Vec<UbiquitousRow>) -> String {
    let mut context = Context::new();
    context.insert("items", &rows);

    let tera = Tera::new("src/outputs/html/templates/*.html")
        .expect("Failed to init Tera with templates/*.html");

    tera.render("ubiquitous.html", &context)
        .expect("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::{render_html, UbiquitousRow};

    #[test]
    fn test_set_github_url() {
        let repo = "owner/repo".to_string();
        let branch = "main".to_string();
        let file_path = "src/lib.rs".to_string();
        let line_number = 42;

        let row = UbiquitousRow::new().set_github_url(repo, branch, file_path, line_number);

        assert_eq!(
            row.github_url,
            "https://github.com/owner/repo/blob/main/src/lib.rs#L42"
        );
    }

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

        let rows = vec![row1, row2];

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
