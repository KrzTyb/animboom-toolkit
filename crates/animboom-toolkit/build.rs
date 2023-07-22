#[cfg(feature = "ffi")]
extern crate cbindgen;

use std::env;

#[cfg(feature = "ffi")]
use std::path::PathBuf;

fn main() {
    if env::var("CARGO_FEATURE_FFI").is_ok() {
        #[cfg(feature = "ffi")]
        generate_ffi_bindings();
    };
}

#[cfg(feature = "ffi")]
fn generate_ffi_bindings() {
    const BINDINGS_FILE_PATH: &str = "target/include";
    const BINDINGS_FILE_NAME: &str = "animboomtoolkit.h";

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut bindings_outdir = PathBuf::new().join(BINDINGS_FILE_PATH);

    if let Ok(custom_bindings_outdir) = env::var("TOOLKIT_BINDINGS_PATH") {
        bindings_outdir = PathBuf::new().join(custom_bindings_outdir);
    };

    let bindings_path = bindings_outdir.join(BINDINGS_FILE_NAME);

    let config = cbindgen::Config {
        header: Some("// Automatically generated C bindings for animboom-toolkit".into()),
        language: cbindgen::Language::C,
        include_guard: Some("ANIMBOOM_TOOLKIT".into()),
        cpp_compat: true,
        include_version: true,
        documentation: true,
        no_includes: true,
        includes: vec![],
        ..Default::default()
    };

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_path);
}
