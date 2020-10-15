extern crate bindgen;

use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("PCANBasic.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let out_dir = env::var("OUT_DIR").unwrap();

    if cfg!(target_env = "gnu") {
        let mut output_lib_path = PathBuf::from(&out_dir);
        output_lib_path.push("PCANBasic.dll");
        fs::copy("lib/PCANBasic.dll", output_lib_path).unwrap();
    } else if cfg!(windows) {
        let machine = if cfg!(target_arch = "x86") {
                "/Machine:X86"
            } else if cfg!(target_arch = "x86_64") {
                "/Machine:X64"
            } else {
                panic!("Unknown architecture");
            };


        let mut output_lib_path = PathBuf::from(&out_dir);
        output_lib_path.push("PCANBasic.lib");
        Command::new("link.exe")
            .args(&["/def:PCANBasic.def", &format!("/OUT:{}", output_lib_path.to_str().unwrap()), machine])
            .status().expect("Could not run link.exe");
    }

    println!("cargo:rustc-link-lib=PCANBasic");
    println!("cargo:rustc-link-search=native={}", out_dir);

}
