extern crate bindgen;

// use std::env;
use std::path::PathBuf;

fn main()
{
    let libdir_path = PathBuf::from("include")
        .canonicalize()
        .expect("cannot canonicalize path");

    // let headers_path = libdir_path.join("baz.h");
    // let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=vim");
    println!("cargo:rustc-link-arg=-lm");
    println!("cargo:rustc-link-arg=-lncurses");
    // println!("cargo:rerun-if-changed={}", headers_path_str);

    // let src = [
    //     "include/foo.c",
    //     "include/bar.c",
    //     "include/blah.c"
    // ];

    // let mut builder = cc::Build::new();
    // let build = builder.files(src.iter());
    // build.compile("baz");

    // let args = ["-DHAVE_CONFIG_H", "-I/home/tomazgomes/Desktop/test/libvim/src/proto"];

    // let bindings = bindgen::Builder::default()
        // .header("include/baz.h")
        // .header("include/libvim/libvim.h")
        // .clang_args(args.iter())
        // .generate()
        // .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
