
fn main() {
    println!("cargo:rustc-link-search=native=./");
    println!("cargo:rustc-link-lib=dylib=proof");
    println!("cargo:rerun-if-changed=libproof.dylib");
}