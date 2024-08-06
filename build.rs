use std::{env, path::PathBuf};

extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-lib=avfilter");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avdevice");
    println!("cargo:rustc-link-lib=avutil");

    let bindings = bindgen::Builder::default()
        .header("headers/libavfilter.h")
        .header("headers/libavformat.h")
        .header("headers/libavcodec.h")
        .header("headers/libavdevice.h")
        .header("headers/libavutil.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_var("FP_NAN")
        .blocklist_var("FP_INFINITE")
        .blocklist_var("FP_ZERO")
        .blocklist_var("FP_SUBNORMAL")
        .blocklist_var("FP_NORMAL")
        .generate()
        .expect("Binding generation error");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out.join("bindings.rs"))
        .expect("Could not write result to bindings.rs");
}
