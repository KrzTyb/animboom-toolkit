extern crate cbindgen;

use std::env;

const BINDINGS_FILE_PATH: &str = "target/include/animboomtoolkit.h";

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

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
        .write_to_file(BINDINGS_FILE_PATH);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
