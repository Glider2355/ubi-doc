use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub fn output_assets(rendered_html: &String, output_path: &Path) {
    // 出力先ディレクトリ（まだなければ作成）
    let assets_dir = output_path.join("ubi-doc");
    fs::create_dir_all(&assets_dir).unwrap();

    // レンダリング結果をHTMLファイルに書き込む
    let mut file = File::create(assets_dir.join("ubiquitous.html")).unwrap();
    file.write_all(rendered_html.as_bytes()).unwrap();

    // script.js / style.css も同フォルダにコピー
    copy_file(
        "src/outputs/html/templates/script.js",
        &assets_dir.join("script.js"),
    );
    copy_file(
        "src/outputs/html/templates/style.css",
        &assets_dir.join("style.css"),
    );
}

fn copy_file(src: &str, dst: &PathBuf) {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::copy(src, dst).unwrap_or_else(|e| {
        panic!("Failed to copy from {:?} to {:?}: {}", src, dst, e);
    });
    println!("Copied {:?} -> {:?}", src, dst);
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

        // HTMLは (output_path)/ubi-doc/ubiquitous.html に作られる
        let ubi_dir = output_path.join("ubi-doc");
        let html_file = ubi_dir.join("ubiquitous.html");
        assert!(html_file.exists(), "ubiquitous.html should exist");
        let content = fs::read_to_string(&html_file).unwrap();
        assert_eq!(content, dummy_html);

        // script.js, style.css も (output_path)/ubi-doc/script.js, (output_path)/ubi-doc/style.css
        let script_path = ubi_dir.join("script.js");
        let style_path = ubi_dir.join("style.css");
        assert!(script_path.exists(), "script.js should be copied");
        assert!(style_path.exists(), "style.css should be copied");
    }
}
