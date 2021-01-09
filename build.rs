extern crate bindgen;

use cmake;

use std::env;
use std::fs;
use std::path;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    //println!("cargo:rerun-if-env-changed=LIBPLCTAGRS_SYS_STATIC");
    println!("cargo:rerun-if-changed=build.rs");
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    
    let include_dir = build_libplctag(&target);
    let bindings = bindgen::Builder::default()
        .header(include_dir.join("libplctag.h").to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    eprintln!("OUT_DIR={:?}", out_path);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}


fn find_target_profile_dir(dir: PathBuf) -> Option<PathBuf> {
    //out dir looks like ...\plctag-rs\target\debug\build\XXXXX
    //profile dir looks like ...\plctag-rs\target\debug\
    let mut dir = dir;
    loop {
        if let Some(p) = dir.parent() {
            let buf = p.to_path_buf();
            if buf.ends_with("build") {
                return Some(buf.parent().unwrap().to_path_buf());
            }
            dir = buf;
        } else {
            return None;
        }
    }
}

fn build_libplctag(target: &str) -> PathBuf {
    let lib_dir = "src/libplctag";
    let install_dir = cmake::Config::new(lib_dir)
        //.define("BUILD_SHARED_LIBS", "OFF")
        .build();
    let includedir = install_dir.join("include");
    let libdir = install_dir.join("lib");
    println!(
        "cargo:rustc-link-search=native={}",
        libdir.to_str().unwrap()
    );
    let libname = "plctag";
    println!("cargo:rustc-link-lib=static={}", libname);
    println!("cargo:root={}", install_dir.to_str().unwrap());
    println!("cargo:include={}", includedir.to_str().unwrap());

    includedir
}
