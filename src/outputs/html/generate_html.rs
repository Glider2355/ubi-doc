use super::{
    output_assets::output_assets, render_html::render_html, ubiquitous_rows::UbiquitousRows,
};
use std::path::Path;

pub fn generate_html(mut ubiquitous_rows: UbiquitousRows, output_path: &Path) {
    ubiquitous_rows.sort();
    let rendered_html: String = render_html(ubiquitous_rows);
    output_assets(&rendered_html, output_path);
}
