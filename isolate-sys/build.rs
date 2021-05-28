use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let lib = pkg_config::Config::new().atleast_version("1.8.2").probe("isolate")
        .expect("Unable to find isolate library using pkg-config");
    let include_path = lib.include_paths.get(0)
        .expect("Retrieved library has no include path");

    let out_path = PathBuf::from(
        std::env::var("OUT_DIR").expect("Cannot retrieve OUT_PATH environment variable")
    );

    bindgen::Builder::default()
        .header(format!("{}", include_path.join("isolate.h").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Error while generating bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Cannot write bindings.rs file");
}