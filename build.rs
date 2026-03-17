use std::env;
use std::path::PathBuf;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(dir).join("lib");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    // On Windows, the dynamic library is usually expected to be named `vosk.lib` (import lib for vosk.dll)
    // or we just link it generically if it's dynamic. The vosk-rs crate automatically requests linking "vosk",
    // so we just need to provide the search path to `lib`.
}
