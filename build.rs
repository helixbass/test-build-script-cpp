use std::env;
use std::path::Path;
extern crate cc;

fn main() {
    let src_dir = Path::new("src");

    let mut cpp_config = cc::Build::new();
    cpp_config.cpp(true);
    cpp_config.include(&src_dir);

    if env::var("TARGET").unwrap() == "wasm32-wasi" {
        cpp_config.flag_if_supported("-fno-exceptions");
    }

    cpp_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");
    let scanner_path = src_dir.join("scanner.cc");
    cpp_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    cpp_config.compile("scanner");
}
