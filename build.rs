
use std::path::PathBuf;

fn main() {
    // Configure the ESP-IDF build system environment variables
    embuild::espidf::sysenv::output();

    // Tell Cargo to rerun this build script if the specified files change
    println!("cargo:rerun-if-changed=src/LX16AServo/mod.rs");
    println!("cargo:rerun-if-changed=components/main/main.h");
    println!("cargo:rerun-if-changed=components/main/main.c");
    println!("cargo:rerun-if-changed=components/main/CMakeLists.txt");
    
    // Let the ESP-IDF build system handle linking
    // This uses embuild's ESP-IDF integration to automatically
    // set up all the link paths and libraries needed
    // Comment this out if you need more control
    
    // Prevent duplicate C++ symbol definitions
    // This tells the linker to allow duplicate definitions and use the first one
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    
    // Explicitly avoid linking to C++ guard symbols that conflict
    println!("cargo:rustc-link-arg=-u");
    println!("cargo:rustc-link-arg=__cxa_guard_dummy");
    
    // For components not automatically found by embuild
    if PathBuf::from("build/esp-idf/main/libmain.a").exists() {
        println!("cargo:rustc-link-search=native=build/esp-idf/main");
        println!("cargo:rustc-link-lib=static=main");
    }

    // Make sure the wrapper header approach is used
    // This is important for cross-compilation with ESP32 as per memory note
    println!("cargo:rustc-cfg=esp32_wrapper_header");
}
