extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=milter");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_var("SMFI_.*")
        .blacklist_item("SMFI_INTERNAL")
        .whitelist_var("MI_.*")
        .whitelist_type("smfi.*")
        .whitelist_var("SMFIF_.*")
        .whitelist_var("SMFIS_.*")
        .whitelist_var("SMFIP_.*")
        .whitelist_function("smfi_.*")
        .generate()
        .expect("Unable to generate libmilter bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write write libmilter bindings");
}