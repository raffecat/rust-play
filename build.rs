extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file: PathBuf = target_dir().join(format!("{}.h", package_name));

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_language(Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file(output_file);
}

/// Helper to find the target directory
fn target_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
}
