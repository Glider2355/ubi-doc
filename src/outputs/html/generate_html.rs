use std::{fs::File, io::Write, path::Path};

pub struct GenerateHtmlParam {
    pub class_name: String,
    pub ubiquitous: String,
    pub context: String,
    pub description: String,
}

pub fn generate_html(ubiquitous_list: Vec<GenerateHtmlParam>, output_path: &Path) {
    let mut html = String::new();
    html.push_str("<html><head><title>Ubiquitous Language</title></head><body>");
    html.push_str("<h1>Ubiquitous Language</h1>");
    html.push_str("<table border='1'>");
    html.push_str(
        "<tr><th>Ubiquitous</th><th>Class Name</th><th>Context</th><th>Description</th></tr>",
    );

    for ubiquitous in ubiquitous_list {
        html.push_str("<tr>");
        html.push_str(&format!("<td>{}</td>", ubiquitous.ubiquitous));
        html.push_str(&format!("<td>{}</td>", ubiquitous.class_name));
        html.push_str(&format!("<td>{}</td>", ubiquitous.context));
        html.push_str(&format!("<td>{}</td>", ubiquitous.description));
        html.push_str("</tr>");
    }

    html.push_str("</table>");
    html.push_str("</body></html>");

    let mut file = File::create(output_path).unwrap();
    file.write_all(html.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_html() {
        let ubiquitous_list = vec![
            GenerateHtmlParam {
                class_name: "User".to_string(),
                ubiquitous: "ユビキタス".to_string(),
                context: "ユーザー".to_string(),
                description: "ユーザー情報".to_string(),
            },
            GenerateHtmlParam {
                class_name: "Item".to_string(),
                ubiquitous: "ユビキタス".to_string(),
                context: "アイテム".to_string(),
                description: "アイテム情報".to_string(),
            },
        ];
        let output_path = Path::new("tests/fixtures/output.html");
        generate_html(ubiquitous_list, output_path);

        let html = std::fs::read_to_string(output_path).unwrap();
        assert!(html.contains("<td>ユビキタス</td>"));
        assert!(html.contains("<td>User</td>"));
        assert!(html.contains("<td>ユーザー</td>"));
        assert!(html.contains("<td>ユーザー情報</td>"));
        assert!(html.contains("<td>アイテム</td>"));
        assert!(html.contains("<td>アイテム情報</td>"));
    }
}
