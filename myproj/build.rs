use std::env;

fn main() {
    // OUT_DIR is the build-output dir for myproj ... which isn't quite right,
    // because libui.so is in the build-output dir for the libui-rs dependency
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out_dir);

    // Even if we hard-code the absolute path to the dir where libui.so is,
    // "cargo run" still gives the error "cannot open shared object file"
    //println!("cargo:rustc-link-search={}", "/path/to/libui-fiddling/myproj/target/debug/build/ui-sys-53c65ff8deff9752/out");

    println!("cargo:rustc-link-lib=dylib=ui");
}
