use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use std::io::Write;

/// Extract include paths from compile_commands.json specific to a component
fn extract_include_paths(component_name: &str) -> Vec<String> {
    let build_dir = PathBuf::from("build");
    let compile_commands_path = build_dir.join("compile_commands.json");

    // If compile_commands.json doesn't exist, we need to inform the user
    if !compile_commands_path.exists() {
        println!("cargo:warning=compile_commands.json not found. Run 'idf.py reconfigure' first to generate it.");
        return vec![];
    }

    let contents = fs::read_to_string(&compile_commands_path)
        .unwrap_or_else(|e| panic!("Failed to read compile_commands.json: {}", e));

    let commands: serde_json::Value = serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("Failed to parse compile_commands.json: {}", e));

    let mut include_paths = HashSet::new();
    let component_path = format!("/components/{}/", component_name);
    let component_path_alt = format!("/components/{}\\", component_name); // Windows-style path

    // Handle both .c and .cpp files to get all possible include paths
    if let Some(commands_array) = commands.as_array() {
        for cmd in commands_array {
            // Filter for files that belong to our component
            if let Some(file) = cmd.get("file").and_then(|f| f.as_str()) {
                if !file.contains(&component_path) && !file.contains(&component_path_alt) {
                    continue; // Skip files not belonging to our component
                }
                
                println!("cargo:warning=Found component file: {}", file);
            } else {
                continue; // Skip entries without a file key
            }

            if let Some(command) = cmd.get("command").and_then(|c| c.as_str()) {
                // Extract -I flags from the command
                let parts: Vec<&str> = command.split_whitespace().collect();
                for (i, part) in parts.iter().enumerate() {
                    if part.starts_with("-I") {
                        // Handle -I/path/to/include format
                        let path = part.trim_start_matches("-I").to_string();
                        include_paths.insert(path);
                    } else if *part == "-I" && i + 1 < parts.len() {
                        // Handle -I /path/to/include format
                        let path = parts[i + 1].to_string();
                        include_paths.insert(path);
                    }
                }
            }
        }
    }

    // Always add this component's specific paths
    let component_src_path = PathBuf::from("components").join(component_name).join("src");
    let component_include_path = PathBuf::from("components").join(component_name).join("include");
    
    if component_src_path.exists() {
        include_paths.insert(component_src_path.to_string_lossy().to_string());
    }
    
    if component_include_path.exists() {
        include_paths.insert(component_include_path.to_string_lossy().to_string());
    }

    // Add Arduino ESP32 specific include paths if they exist - required for components that use Arduino
    let arduino_esp32_paths = [
        "build/esp-idf/components/arduino-esp32/cores/esp32",
        "build/esp-idf/components/arduino-esp32/variants/esp32",
        "build/esp-idf/components/arduino-esp32/libraries",
        "build/esp-idf/arduino_tinyusb/include",
    ];

    for path in arduino_esp32_paths.iter() {
        if Path::new(path).exists() {
            include_paths.insert(path.to_string());
        }
    }

    // Add ESP-IDF config path which is always needed
    let config_path = build_dir.join("config");
    if config_path.exists() {
        include_paths.insert(config_path.to_string_lossy().to_string());
    }

    // Get toolchain includes which are always needed
    if let Ok(output) = std::process::Command::new("xtensa-esp32-elf-gcc")
        .args(["-print-file-name=include"])
        .output() {
        if output.status.success() {
            let toolchain_include = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !toolchain_include.is_empty() {
                include_paths.insert(toolchain_include);
            }
        }
    }

    // Add IDF_PATH/components which is often needed
    if let Ok(idf_path) = env::var("IDF_PATH") {
        let idf_components = PathBuf::from(idf_path).join("components");
        if idf_components.exists() {
            include_paths.insert(idf_components.to_string_lossy().to_string());
        }
    }

    println!("cargo:warning=Found {} include paths for component {}", include_paths.len(), component_name);
    include_paths.into_iter().collect()
}


/// Generate bindings for a specific component
fn generate_binding(
    component_name: &str,
    header_files: &[&str],
    allowlist_types: &[&str],
    opaque_types: &[&str]
) {
    println!("cargo:warning=Generating bindings for component: {}", component_name);
    
    // Get the output directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Get include paths from compile_commands.json for the component
    let include_paths = extract_include_paths(component_name);
    
    // Convert include paths to clang args (i.e., -I<path>)
    let include_args: Vec<String> = include_paths
        .into_iter()
        .map(|path| format!("-I{}", path))
        .collect();

    // Start with basic args for C++ compilation
    let mut clang_args = vec![
        "-x".to_string(), 
        "c++".to_string(), 
        "-std=c++14".to_string(),
        "--target=x86_64-linux".to_string(),
        "-DARDUINO_ARCH_ESP32".to_string(),
        // Add definitions to handle missing system headers
        "-D__MACHINE_ENDIAN_H__".to_string(),      // Skip machine/endian.h
        "-D_ENDIAN_H".to_string(),                  // Skip endian.h if needed
        "-D__BYTE_ORDER=1234".to_string(),         // Define byte order as little endian (common for ESP32)
        "-D__LITTLE_ENDIAN=1234".to_string(),      // Define little endian macro
        "-D__BIG_ENDIAN=4321".to_string(),         // Define big endian macro
    ];
    
    // Add the include paths
    clang_args.extend(include_args);
    
    // Start building the bindgen configuration
    let mut builder = bindgen::Builder::default();
    
    // Add all header files, filtering if needed
    for &header in header_files {
        builder = builder.header(format!("components/{}/{}", component_name, header));
    }
    
    // Add allowlist types
    for &type_name in allowlist_types {
        builder = builder.allowlist_type(type_name);
    }
    
    // Add opaque types
    for &type_name in opaque_types {
        builder = builder.opaque_type(type_name);
    }
    
    // Complete the bindgen configuration with common settings
    let bindings = builder
        .clang_args(&clang_args)
        .enable_cxx_namespaces()
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect(&format!("Unable to generate bindings for {}", component_name));

    // Create the component-specific output file
    let binding_file = format!("{}.rs", component_name);
    bindings
        .write_to_file(out_path.join(&binding_file))
        .expect(&format!("Couldn't write bindings for {}!", component_name));
    
    println!("cargo:warning=Bindings for {} generated successfully", component_name);

    // Make sure the linker knows about our component
    println!("cargo:rustc-link-search=native=components/{}/build", component_name);
    println!("cargo:rustc-link-lib=static={}", component_name);
}

fn main() {
    // Tell Cargo to re-run build script if these files change
    println!("cargo:rerun-if-changed=build/compile_commands.json");
    println!("cargo:rerun-if-env-changed=ESP_IDF_PATH");
    println!("cargo:rerun-if-env-changed=IDF_PATH");
    
    // Generate bindings for lx16a-servo component
    println!("cargo:rerun-if-changed=components/lx16a-servo/src/lx16a-servo.h");
    generate_binding(
        "lx16a-servo",
        &["src/lx16a-servo.h"],
        &["LX16AServo", "LX16ABus", "Serial"],
        &["std::.*", "LX16ABus", "LX16AServo", "Serial"]
    );
    
    // Always link with stdc++ which is required for C++ bindings
    println!("cargo:rustc-link-lib=stdc++");
}
