extern crate bindgen;

use std::path::PathBuf;

fn main()
{
    let libdir_path = PathBuf::from("include")
        .canonicalize()
        .expect("cannot canonicalize path");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=vim");
    println!("cargo:rustc-link-arg=-lm");
    println!("cargo:rustc-link-arg=-lncurses");
}
