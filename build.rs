use std::fs::{self};
use std::path::{Path, PathBuf};

use minify_html::{Cfg, minify};

use minify_js::{minify as js_minify, Session, TopLevelMode};

fn main() {
    let mut cfg = Cfg::new();
    cfg.ensure_spec_compliant_unquoted_attribute_values = true;
    cfg.keep_closing_tags = true;
    cfg.minify_css = true;
    cfg.minify_js = true;
    cfg.remove_bangs = true;
    cfg.remove_processing_instructions = true;

    let fname = "static/index.html";
    let new_fname = generate_output_filename(fname);

    let contents = fs::read_to_string(fname).unwrap();
    let minified = minify(contents.as_bytes(), &cfg);
    fs::write(new_fname, minified).unwrap();

    let fname = "static/form.js";
    let new_fname = generate_output_filename(fname);

    let contents = fs::read_to_string(fname).unwrap();
    let session = Session::new();
    let mut out = Vec::new();
    js_minify(&session, TopLevelMode::Global, contents.as_bytes(), &mut out).unwrap();
    fs::write(new_fname, out).unwrap();
}

fn generate_output_filename(input_filename: &str) -> PathBuf {
    let input_path = Path::new(input_filename);
    let extension = input_path.extension().unwrap().to_str().unwrap();
    let mut buf = input_path.to_path_buf();
    buf.set_extension(format!("min.{}", extension));
    buf
}