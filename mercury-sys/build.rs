#![cfg(target_os = "linux")]

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let dst = cmake::Config::new("mercury")
        .profile("Release")
        .define("MERCURY_USE_BOOST_PP", "ON")
        .define("NA_USE_OFI", "ON")
        .build();
    let build = dst.join("build");

    println!("cargo:root={}", build.display());
    println!("cargo:rustc-link-lib=static=mercury");
    println!("cargo:rustc-link-lib=static=mercury_util");
    println!("cargo:rustc-link-lib=static=na");
    println!("cargo:rustc-link-search={}/lib", dst.display());

    // For libfabric.
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=fabric");

    let include_dir = dst.join("include");
    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_dir.display()))
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
