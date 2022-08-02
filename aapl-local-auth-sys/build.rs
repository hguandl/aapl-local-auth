use std::env;
use std::path::PathBuf;

fn sdk_path() -> Result<String, std::io::Error> {
    use std::process::Command;
    let output = Command::new("xcrun")
        .args(&["--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=framework=LocalAuthentication");

    let sdk_path = sdk_path().expect("failed to get SDK path");
    let clang_args = vec!["-x", "objective-c", "-fblocks", "-isysroot", &sdk_path];

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_args(&clang_args)
        .objc_extern_crate(true)
        .layout_tests(false)
        .block_extern_crate(true)
        .generate_block(true)
        .blocklist_item("id")
        .blocklist_item("timezone")
        .blocklist_item("FndrOpaqueInfo")
        .blocklist_item("HFS.*Catalog.*")
        .blocklist_item(".+Deprecated")
        // The input header we would like to generate
        // bindings for.
        .header_contents(
            "LocalAuthentication.h",
            "#import <LocalAuthentication/LocalAuthentication.h>",
        )
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
