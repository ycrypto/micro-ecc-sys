use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = cc::Build::new();
    let builder = builder
        .flag("-std=c11")
        .file("micro-ecc/uECC.c")
    ;

    builder.compile("micro-ecc-sys");

    let bindings = bindgen::Builder::default()
        .header("micro-ecc/uECC.h")
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
