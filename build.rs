// build.rs
fn main() {
    // Re-run this build script if boot.S changes
    println!("cargo:rerun-if-changed=src/boot.S");
    
    // Assemble src/boot.S using the cc crate
    cc::Build::new()
        .file("src/boot.S")
        .compile("boot");
}