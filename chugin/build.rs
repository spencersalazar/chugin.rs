extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=chuck/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/wrapper.h")
        // force C++
        .clang_arg("-x").clang_arg("c++")
        .allowlist_type("Chuck_DL_Query")
        .allowlist_type("Chuck_DL_Api::Api")
        .allowlist_type("Chuck_Object")
        .opaque_type("Chuck_Carrier")
        .opaque_type("Chuck_Compiler")
        .opaque_type("Chuck_VM")
        .opaque_type("Chuck_Env")
        .opaque_type("Chuck_DLL")
        .opaque_type("Chuck_DL_Class")
        .opaque_type("Chuck_DL_Func")
        .opaque_type("Chuck_VTable")
        .opaque_type("std::string")
        .opaque_type("std::vector")
        .opaque_type("std::map")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
