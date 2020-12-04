use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

struct Builder {
    out_dir: PathBuf,
    include_dir: PathBuf,
    src_libnetmap_dir: PathBuf,
}

impl Builder {
    pub fn new() -> Self {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let include_dir = PathBuf::from("netmap/sys");
        let src_libnetmap_dir = PathBuf::from("netmap/libnetmap");

        Self {
            out_dir,
            include_dir,
            src_libnetmap_dir,
        }
    }

    pub fn build(&mut self) {
        self.rerun_if_wrapper_changed();
        self.export_include_path();
        self.generate_bindings();

        self.rerun_if_libnetmap_file_changed();
        self.make_libnetmap();
        self.link_libnetmap();
    }

    fn export_include_path(&self) {
        println!("cargo:include={}", abs_path(&self.include_dir));
    }

    fn rerun_if_libnetmap_file_changed(&self) {
        for file in self.src_libnetmap_dir.read_dir().unwrap() {
            println!("cargo:rerun-if-changed={}", abs_path(&file.unwrap().path()));
        }
    }

    fn rerun_if_wrapper_changed(&self) {
        // Tell cargo to invalidate crate if changed
        println!("cargo:rerun-if-changed=wrapper.h");
    }

    fn make_libnetmap(&self) {
        cp_r(&self.src_libnetmap_dir, &self.out_dir);
        process::Command::new("make").current_dir(self.out_dir.join("libnetmap"));
    }

    fn generate_bindings(&self) {
        bindgen::Builder::default()
            .header("src/wrapper.h")
            .clang_arg(format!("-I{}", self.include_dir.to_str().unwrap()))
            .clang_arg(format!("-I{}", self.src_libnetmap_dir.to_str().unwrap()))
            //.default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Failed to generate bindings")
            .write_to_file(self.out_dir.join("bindings.rs"))
            .expect("Failed to write bindings");
    }

    fn link_libnetmap(&self) {
        println!(
            "cargo:rustc-link-search=native={}",
            self.out_dir.join("libnetmap").to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=static=netmap",);
    }
}

fn abs_path(pathbuf: &PathBuf) -> String {
    pathbuf
        .as_path()
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
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

fn main() {
    Builder::new().build();
}
