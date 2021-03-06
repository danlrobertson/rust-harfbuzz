extern crate pkg_config;

use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    if env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::find_library("harfbuzz").is_ok() {
            return
        }
    }

    assert!(Command::new("make")
        .args(&["-R", "-f", "makefile.cargo", &format!("-j{}", env::var("NUM_JOBS").unwrap())])
        .status()
        .unwrap()
        .success());

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_dir.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=static=harfbuzz");
    println!("cargo:rustc-link-lib=stdc++");
}
