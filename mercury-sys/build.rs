#![cfg(target_os = "linux")]

fn main() {
    build_mercury();
}

fn build_mercury() {
    let dst = cmake::Config::new("mercury")
        .profile("Release")
        .build_target("mercury")
        .build();
    let build = dst.join("build");

    println!("cargo:root={}", build.display());
    println!("cargo:rustc-link-lib=static=mercury");
    println!(
        "cargo:rustc-link-search=native={}",
        build.join("bin").display()
    );
}
