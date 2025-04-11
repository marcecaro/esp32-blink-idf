use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    // Configure the ESP-IDF build system environment variables
    embuild::espidf::sysenv::output();

    // Always watch for changes in Rust modules
    println!("cargo:rerun-if-changed=src/LX16AServo/mod.rs");
    
    // Discover and watch for changes in all components
    watch_components_directory();
    
    // Prevent duplicate C++ symbol definitions
    // This tells the linker to allow duplicate definitions and use the first one
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    
    // Explicitly avoid linking to C++ guard symbols that conflict
    println!("cargo:rustc-link-arg=-u");
    println!("cargo:rustc-link-arg=__cxa_guard_dummy");
    
    // Link all discovered component libraries
    link_component_libraries();

    // Make sure the wrapper header approach is used
    // This is important for cross-compilation with ESP32 as per memory note
    println!("cargo:rustc-cfg=esp32_wrapper_header");
}

/// Scans all components in the components directory and adds cargo:rerun-if-changed
/// entries for relevant files.
fn watch_components_directory() {
    let components_dir = PathBuf::from("components");
    if !components_dir.exists() {
        println!("cargo:warning=Components directory not found");
        return;
    }

    // Always rerun if the components directory structure changes
    println!("cargo:rerun-if-changed=components");

    // Try to read the components directory
    if let Ok(entries) = fs::read_dir(&components_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let component_name = path.file_name().unwrap().to_string_lossy().to_string();
                println!("cargo:warning=Found component: {}", component_name);

                // Watch for changes to common file patterns in this component
                watch_component_files(&path);
            }
        }
    }
}

/// Add cargo:rerun-if-changed entries for common file types in a component
fn watch_component_files(component_path: &Path) {
    // Always watch CMakeLists.txt
    let cmake_path = component_path.join("CMakeLists.txt");
    if cmake_path.exists() {
        println!("cargo:rerun-if-changed={}", cmake_path.to_string_lossy());
    }

    // Watch C/C++ source and header files
    for extension in &[".c", ".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx"] {
        if let Ok(entries) = fs::read_dir(component_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == &extension[1..] {
                            println!("cargo:rerun-if-changed={}", path.to_string_lossy());
                        }
                    }
                }
            }
        }
    }
}

/// Link all component libraries found in build/esp-idf/[component_name]/lib[component_name].a
fn link_component_libraries() {
    let build_dir = PathBuf::from("build/esp-idf");
    if !build_dir.exists() {
        // Build directory doesn't exist yet, which is fine
        return;
    }

    if let Ok(entries) = fs::read_dir(&build_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let component_name = path.file_name().unwrap().to_string_lossy().to_string();
                let lib_path = path.join(format!("lib{}.a", component_name));
                
                if lib_path.exists() {
                    println!("cargo:rustc-link-search=native={}", path.to_string_lossy());
                    println!("cargo:rustc-link-lib=static={}", component_name);
                    println!("cargo:warning=Linked component library: {}", component_name);
                }
            }
        }
    }
}
