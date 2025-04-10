// build.rs for ESP32 project with C++ components
use std::env;
use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};



fn main() {
    println!("cargo:warning=Building ESP32 project with C++ components");
    
    // Get current directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    // Use ESP-IDF build system's environment variables for linking
    if let Ok(idf_path) = env::var("IDF_PATH") {
        println!("cargo:warning=Using ESP-IDF from: {}", idf_path);
    } else {
        println!("cargo:warning=IDF_PATH not set, using project-export.sh");
    }
    
    // Try to use embuild to propagate ESP-IDF configurations
    if let Ok(_) = embuild::build::CfgArgs::output_propagated("ESP_IDF") {
        println!("cargo:warning=ESP-IDF configuration propagated successfully");
    } else {
        println!("cargo:warning=ESP-IDF configuration not found. Make sure to run '. project-export.sh' first");
    }
    
    // Try to use embuild to propagate ESP-IDF link arguments
    if let Ok(_) = embuild::build::LinkArgs::output_propagated("ESP_IDF") {
        println!("cargo:warning=ESP-IDF link arguments propagated successfully");
    } else {
        println!("cargo:warning=ESP-IDF link arguments not found. Make sure to run '. project-export.sh' first");
    }
    
    // Add ESP-IDF build directories and our C++ components to the linker search paths
    setup_esp_idf_and_component_linking(&current_dir);
    
    // Tell cargo to rebuild if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=sdkconfig");
    println!("cargo:rerun-if-changed=sdkconfig.defaults");
    println!("cargo:rerun-if-changed=components/");

}


// Function to run a command and stream its output in real-time
#[allow(unused)]
fn run_command_with_real_time_output(cmd: &str) -> std::process::ExitStatus {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to spawn command: {}", cmd));

    // Handle stdout in real-time
    if let Some(stdout) = child.stdout.take() {
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                println!("cargo:warning={}", line);
                std::io::stdout().flush().unwrap();
            }
        }
    }

    // Handle stderr in real-time
    if let Some(stderr) = child.stderr.take() {
        let stderr_reader = BufReader::new(stderr);
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                println!("cargo:warning={}", line);
                std::io::stdout().flush().unwrap();
            }
        }
    }

    // Wait for the process to complete and get the status
    child.wait().expect(&format!("Failed to wait for command: {}", cmd))
}

// Setup ESP-IDF and component linking
fn setup_esp_idf_and_component_linking(project_dir: &Path) {
    let build_dir = project_dir.join("build");
    if !build_dir.exists() {
        println!("cargo:warning=Build directory not found. Please run '. project-export.sh && idf.py build' first");
        return;
    }

    // Try to extract linker flags from ESP-IDF build system
    let ldgen_libs_file = build_dir.join("ldgen_libraries.in");
    let ldgen_libraries = if ldgen_libs_file.exists() {
        match fs::read_to_string(&ldgen_libs_file) {
            Ok(content) => {
                println!("cargo:warning=Using ldgen_libraries.in for ESP-IDF libraries");
                content
            },
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    // 1. Add component libraries and directories
    println!("cargo:warning=Adding ESP-IDF component libraries");

    // Add all library directories found in the build directory
    println!("cargo:rustc-link-search={}", build_dir.display());
    find_and_add_lib_dirs(&build_dir);

    // 2. Parse and add libraries from ldgen_libraries.in if available
    for line in ldgen_libraries.lines() {
        let line = line.trim();
        if !line.is_empty() && !line.starts_with('#') {
            if let Some(lib_path) = Path::new(line).parent() {
                // Only add non-empty paths
                let path_str = lib_path.to_string_lossy();
                if !path_str.is_empty() {
                    println!("cargo:rustc-link-search={}", lib_path.display());
                }
                
                // Extract the library name without lib prefix and .a suffix
                if let Some(file_name) = Path::new(line).file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.starts_with("lib") && file_name_str.ends_with(".a") {
                            let lib_name = &file_name_str[3..file_name_str.len()-2];
                            println!("cargo:rustc-link-lib={}", lib_name);
                        }
                    }
                }
            }
        }
    }

    // 3. Automatically discover and link component libraries
    println!("cargo:warning=Automatically discovering components from ./components");
    
    // Always link with main component
    println!("cargo:warning=Linking with component: main");
    println!("cargo:rustc-link-lib=main");
    
    // Discover components from the components directory
    let components_dir = project_dir.join("components");
    if components_dir.exists() && components_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&components_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Extract component name from directory name
                    if let Some(component_name) = path.file_name().and_then(|n| n.to_str()) {
                        println!("cargo:warning=Linking with discovered component: {}", component_name);
                        println!("cargo:rustc-link-lib={}", component_name);
                    }
                }
            }
        } else {
            println!("cargo:warning=Could not read components directory");
        }
    } else {
        println!("cargo:warning=Components directory not found at {}", components_dir.display());
    }

    // 4. Add essential ESP-IDF system libraries
    let essential_libs = [
        "esp_system", "freertos", "hal", "esp_rom", "esp_common", "heap",
        "log", "soc", "esp_hw_support", "pthread", "spi_flash", "newlib",
        "xtensa", "c", "m", "gcc", "stdc++", "wpa_supplicant", "mbedtls", "lwip"
    ];

    println!("cargo:warning=Adding ESP-IDF system libraries");
    for lib in essential_libs.iter() {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // 5. Add ESP-IDF specific linker flags for proper linking
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-u __cxa_guard_dummy");
    println!("cargo:rustc-link-arg=-u __cxx_fatal_exception");
    println!("cargo:rustc-link-arg=-Wl,--undefined=uxTopUsedPriority");
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");

    // 6. Handle FreeRTOS special case with whole-archive
    println!("cargo:rustc-link-arg=-Wl,--whole-archive");
    println!("cargo:rustc-link-arg=-lfreertos");
    println!("cargo:rustc-link-arg=-Wl,--no-whole-archive");

    // 7. Allow multiple definitions to handle ESP-IDF linking quirks
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
}

// Helper function to recursively find and add library directories
fn find_and_add_lib_dirs(dir: &Path) {
    if !dir.is_dir() {
        return;
    }
    
    // Check if this directory contains .a files
    let mut has_lib_files = false;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "a") {
                has_lib_files = true;
                break;
            }
        }
    }
    
    // If this directory has .a files, add it as a library search path
    if has_lib_files {
        println!("cargo:rustc-link-search={}", dir.display());
    }
    
    // Recurse into subdirectories, but avoid going too deep to prevent stack overflow
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip some directories to avoid excessive recursion
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !dir_name.starts_with(".") && dir_name != "node_modules" {
                    find_and_add_lib_dirs(&path);
                }
            }
        }
    }
}

