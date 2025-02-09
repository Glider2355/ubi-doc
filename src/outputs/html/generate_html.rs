use super::{render_html::render_html, ubiquitous_row::UbiquitousRow};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn generate_html(ubiquitous_rows: Vec<UbiquitousRow>, output_path: &Path) {
    // 出力先ディレクトリが存在しない場合は作成
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let rendered_html = render_html(ubiquitous_rows);

    // レンダリング結果をファイルに書き込む
    let mut file = File::create(output_path).unwrap();
    file.write_all(rendered_html.as_bytes()).unwrap();
}
