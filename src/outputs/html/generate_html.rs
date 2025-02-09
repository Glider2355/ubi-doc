use super::{
    output_assets::output_assets, render_html::render_html, ubiquitous_row::UbiquitousRow,
};
use std::path::Path;

pub fn generate_html(ubiquitous_rows: Vec<UbiquitousRow>, output_path: &Path) {
    let rendered_html: String = render_html(ubiquitous_rows);
    output_assets(&rendered_html, output_path);
}
