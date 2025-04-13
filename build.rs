use std::path::Path;

fn main() {
    // Set up ESP-IDF build environment
    // This is critical for esp-idf-sys to generate proper bindings
    embuild::build::CfgArgs::output_propagated("ESP_IDF").unwrap();
    embuild::build::LinkArgs::output_propagated("ESP_IDF").unwrap();
    
    // Ensure we rebuild if the lib.rs changes
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/main_extern/mod.rs");
    
    // Fix for multiple definition errors
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    
    // These flags help resolve VFS conflicts
    println!("cargo:rustc-link-arg=-u");
    println!("cargo:rustc-link-arg=__cxa_guard_dummy");
    
    // Explicitly exclude problematic symbols from libnosys
    println!("cargo:rustc-link-arg=-Wl,--exclude-libs=libnosys.a");
}
