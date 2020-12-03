use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

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

fn cp_r(from: &PathBuf, to: &PathBuf) {
    if fs::metadata(from).unwrap().is_file() {
        let to_file = to.join(from.file_name().unwrap());
        fs::copy(from, to_file).unwrap();
    } else {
        let new_dir_in_to = to.join(from.file_name().unwrap());
        fs::create_dir(&new_dir_in_to)
            .unwrap_or_else(|_| println!("Failed to create: {}", new_dir_in_to.to_str().unwrap()));
        for file_object in from.read_dir().unwrap() {
            let file_object = file_object.unwrap();
            cp_r(&file_object.path(), &new_dir_in_to);
        }
    }
}

fn prepare_libnetmap(out_dir: &PathBuf) {
    cp_r(&PathBuf::from("netmap/libnetmap"), out_dir);
    process::Command::new("make").current_dir(out_dir.join("libnetmap"));
}

fn main() {
    // Tell cargo to invalidate crate if changed
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_dir = PathBuf::from("netmap/sys");
    println!(
        "cargo:include={}",
        include_dir
            .as_path()
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    generate_bindings(&include_dir, &out_dir);

    prepare_libnetmap(&out_dir);
}
