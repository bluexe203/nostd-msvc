fn main() {
    // Call the linker configuration logic from our helper crate
    nostd_msvc_build::configure_linker();

    // Re-run this script if build.rs or the helper library changes
    println!("cargo:rerun-if-changed=build.rs");
}
