use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the kernel config changes
    println!("cargo:rerun-if-changed=../../.config");
    println!("cargo:rerun-if-changed=./bindgen_wrapper.h");

    // Regenerate the wrapper
    Command::new("bash")
        .arg("gen_wrapper.sh")
        .output()
        .expect("Unable to run gen_wrapper.sh");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("bindgen_wrapper.h")
        // set the root directory for nested `#include`s
        .clang_arg("-F../../include/")
        // prefix types with `core::ffi` for a `no_std` environment
        .ctypes_prefix("core::ffi")
        // `core` instead of `libstd`
        .use_core()
        .derive_default(true)
        // Make an actual enum for this type
        .rustified_enum("nk_gpu_dev_bit_blit_op")
        .rust_target(bindgen::RustTarget::Nightly)
        // use with caution - NK's C code is built with GCC
        // whereas bindgen (and rustc) use clang.
        //.emit_builtins()
        //
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings (did you run gen_wrapper.sh?)");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
