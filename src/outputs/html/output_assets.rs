use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn output_assets(rendered_html: &String, output_path: &Path) {
    fs::create_dir_all(output_path).unwrap();

    let mut file = File::create(output_path.join("index.html")).unwrap();
    file.write_all(rendered_html.as_bytes()).unwrap();

    // コンパイル時に設定されるリポジトリのルートパスを取得
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let templates_dir = Path::new(manifest_dir).join("src/outputs/html/templates");

    // script.js と style.css をコピー
    copy_file(
        &templates_dir.join("script.js"),
        &output_path.join("script.js"),
    );
    copy_file(
        &templates_dir.join("style.css"),
        &output_path.join("style.css"),
    );
}

fn copy_file(src: &Path, dst: &PathBuf) {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::copy(src, dst).unwrap_or_else(|e| {
        panic!("Failed to copy from {:?} to {:?}: {}", src, dst, e);
    });
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_output_assets_with_tempfile() {
        let tmp_dir = tempdir().unwrap();
        let output_path = tmp_dir.path().to_path_buf();

        let dummy_html = "<html><body>Test Content</body></html>".to_string();
        output_assets(&dummy_html, &output_path);

        let html_file = output_path.join("index.html");
        assert!(html_file.exists(), "ubiquitous.html should exist");
        let content = fs::read_to_string(&html_file).unwrap();
        assert_eq!(content, dummy_html);

        let script_path = output_path.join("script.js");
        let style_path = output_path.join("style.css");
        assert!(script_path.exists(), "script.js should be copied");
        assert!(style_path.exists(), "style.css should be copied");
    }
}
