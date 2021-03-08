use std::path::PathBuf;

fn main() {
    //Link to libaiff
    println!("cargo:rustc-link-lib=aiff");
    //Recompile on wrapper change
    println!("cargo:rerun-if-changed=wrapper.h");

    //Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Error generating bindings!");

    //Write
    let path = PathBuf::from("src").join("bindings.rs");
    bindings
        .write_to_file(path)
        .expect("Error writing bindings!");
}
