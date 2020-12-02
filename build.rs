use std::env;
use std::path::PathBuf;

fn generate_bindings(include_dir: &PathBuf, out_dir: &PathBuf) {
    bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        //.default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write bindings");
}

fn main() {
    // Tell cargo to invalidate crate if changed
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_dir = PathBuf::from("netmap/sys");
    println!("cargo:include={}", include_dir.to_str().unwrap());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    generate_bindings(&include_dir, &out_dir);
}
