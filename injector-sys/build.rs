extern crate bindgen;
use std::env;
use std::path::PathBuf;
use bindgen::RustTarget;
use std::process::Command;

pub fn main(){
    let out_dir = env::var("OUT_DIR").unwrap();

    std::fs::create_dir_all("injector/build").unwrap();
    Command::new("cmake").arg("../").current_dir("injector/build").status().unwrap();
    Command::new("cmake").args(["--build", "."]).current_dir("injector/build").status().unwrap();
    
    std::fs::copy("injector/build/Debug/STATIC.lib", &format!("{}/{}", out_dir, "injector-static.lib")).unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=injector-static");
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