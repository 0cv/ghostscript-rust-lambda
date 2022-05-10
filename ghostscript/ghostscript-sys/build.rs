use std::env::var;

fn main() {
    match var("LOCAL_LIB") {
        Ok(value) => println!("cargo:rustc-link-search={}", value),
        _ => ()
    };
    println!("cargo:rustc-link-search=/src/ghostscript/sobin");
    println!("cargo:rustc-link-lib=gs");

    // You can override ghostscript library name/location using
    // [target.<your triple>.ghostscript] section in ".cargo/config".
    // See cargo docs for "links" attribute for how.

    // Maybe this build script should download precompiled ghostscript
    // or build it from sources. Suggestions of better ways to do that are welcome.
}
