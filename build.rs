use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate crate if changed
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_dir = PathBuf::from("netmap/sys");
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        //.default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    println!("cargo:include={}", include_dir.to_str().unwrap());

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
