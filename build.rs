extern crate cc;

use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("cpp-src/hellocpp.cc")
        .compile("libhellopp.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hellocpp.cc");
}
