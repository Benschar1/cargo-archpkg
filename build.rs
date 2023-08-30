fn main() {
    // could use pkg_config for this
    println!("cargo:rustc-link-lib=ssh2");
    println!("cargo:rustc-link-lib=git2");
    println!("cargo:rerun-if-changed=build.rs");
}
