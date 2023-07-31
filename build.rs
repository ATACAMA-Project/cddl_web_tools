use std::fs;
use std::path::{Path, PathBuf};

use minify_html::{Cfg, minify as minify2};

use minifier::js::minify;

fn main() {
    let mut cfg = Cfg::new();
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    cfg.keep_closing_tags = true;
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.remove_bangs = true;
    cfg.remove_processing_instructions = true;

    minify_and_write("static/index.html", |str| minify2(str.as_bytes(), &cfg));
    minify_and_write("static/form.js", |str| minify(str.as_str()).to_string());
}

fn minify_and_write<F, C: AsRef<[u8]>>(fname: &str, minify_fn: F)
    where F: Fn(String) -> C, {
    let new_fname = generate_output_filename(fname);
    let contents = fs::read_to_string(fname).unwrap();
    let minified = minify_fn(contents);
    fs::write(new_fname, minified).unwrap();
}

fn generate_output_filename(input_filename: &str) -> PathBuf {
    let input_path = Path::new(input_filename);
    let extension = input_path.extension().unwrap().to_str().unwrap();
    let mut buf = input_path.to_path_buf();
    buf.set_extension(format!("min.{}", extension));
    buf
}
