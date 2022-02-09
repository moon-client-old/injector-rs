extern crate bindgen;
use std::env;
use std::path::PathBuf;
use bindgen::RustTarget;

pub fn main(){
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=injector-static.lib");
    println!("cargo:rerun-if-changed=build.rs");
    let bindings = bindgen::Builder::default()
        .header("injector/include/injector.h")
        .rust_target(RustTarget::Nightly)
        .blocklist_type("_IMAGE_TLS_DIRECTORY64")
        .blocklist_type("IMAGE_TLS_DIRECTORY64")
        .blocklist_type("IMAGE_TLS_DIRECTORY")
        .blocklist_type("PIMAGE_TLS_DIRECTORY64")
        .blocklist_type("PIMAGE_TLS_DIRECTORY")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}