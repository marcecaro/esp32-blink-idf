use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

/// Extract include paths and defines from compile_commands.json
fn extract_include_paths(component_name: &str) -> (Vec<String>, Vec<String>) {
    let build_dir = PathBuf::from("build");
    let compile_commands_path = build_dir.join("compile_commands.json");

    // If compile_commands.json doesn't exist, we need to inform the user
    if !compile_commands_path.exists() {
        println!("cargo:warning=compile_commands.json not found. Run 'idf.py reconfigure' first to generate it.");
        return (vec![], vec![]);
    }

    let contents = fs::read_to_string(&compile_commands_path)
        .unwrap_or_else(|e| panic!("Failed to read compile_commands.json: {}", e));

    let commands: serde_json::Value = serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("Failed to parse compile_commands.json: {}", e));

    let mut include_paths = HashSet::new();
    let mut defines = HashSet::new();
    let component_path = format!("/components/{}/", component_name);
    
    // Find a C++ file from our component and use its exact command line
    // This ensures we get the right include paths and defines
    if let Some(commands_array) = commands.as_array() {
        for cmd in commands_array {
            // Check if this is a file from our component
            if let Some(file) = cmd.get("file").and_then(|f| f.as_str()) {
                if file.contains(&component_path) && file.ends_with(".cpp") {
                    // We found a C++ file from our component, use its exact command line
                    if let Some(command) = cmd.get("command").and_then(|c| c.as_str()) {
                        println!("cargo:warning=Found command for our component: {}", file);
                        println!("cargo:warning=Command: {}", command.replace("\n", " "));
                        
                        // Extract all flags (-I and -D) from the command
                        let parts: Vec<&str> = command.split_whitespace().collect();
                        for (i, part) in parts.iter().enumerate() {
                            // Handle include paths (-I flags)
                            if part.starts_with("-I") && part.len() > 2 {
                                // Handle -I/path/to/include format (no space after -I)
                                let path = part[2..].to_string();
                                include_paths.insert(path.clone());
                                
                            } else if *part == "-I" && i + 1 < parts.len() {
                                // Handle -I /path/to/include format (space after -I)
                                let path = parts[i + 1].to_string();
                                include_paths.insert(path.clone());
                            }
                            
                            // Handle define flags (-D flags)
                            else if part.starts_with("-D") && part.len() > 2 {
                                // Handle -DFLAG=value format (no space after -D)
                                let mut define = part[2..].to_string();
                                // Fix escaped quotes in defines
                                define = define.replace("\\\"", "\""); // Convert \" to "
                                defines.insert(define.clone());
                                
                            } else if *part == "-D" && i + 1 < parts.len() {
                                // Handle -D FLAG=value format (space after -D)
                                let mut define = parts[i + 1].to_string();
                                // Fix escaped quotes in defines
                                define = define.replace("\\\"", "\""); // Convert \" to "
                                defines.insert(define.clone());
                            }
                        }
                        
                        // Once we find one C++ file from our component, that's enough
                        break;
                    }
                }
            }
        }
    }
    
    // If we didn't find a specific command for our component, scan all commands
    if include_paths.is_empty() {
        println!("cargo:warning=No specific C++ file found for component, scanning all commands...");
        if let Some(commands_array) = commands.as_array() {
            for cmd in commands_array {
                // Get the command string
                if let Some(command) = cmd.get("command").and_then(|c| c.as_str()) {
                    // Check if this is a relevant command (C/C++ compilation)
                    if !command.contains("-c ") && !command.contains("-E ") {
                        continue;
                    }
                    
                    // Extract all flags (-I and -D) from the command
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    for (i, part) in parts.iter().enumerate() {
                        // Handle include paths (-I flags)
                        if part.starts_with("-I") && part.len() > 2 {
                            // Handle -I/path/to/include format (no space after -I)
                            let path = part[2..].to_string();
                            include_paths.insert(path.clone());
                            
                        } else if *part == "-I" && i + 1 < parts.len() {
                            // Handle -I /path/to/include format (space after -I)
                            let path = parts[i + 1].to_string();
                            include_paths.insert(path.clone());
                        }
                        
                        // Handle define flags (-D flags)
                        else if part.starts_with("-D") && part.len() > 2 {
                            // Handle -DFLAG=value format (no space after -D)
                            let mut define = part[2..].to_string();
                            // Fix escaped quotes in defines
                            define = define.replace("\\\"", "\""); // Convert \" to "
                            defines.insert(define.clone());
                            
                        } else if *part == "-D" && i + 1 < parts.len() {
                            // Handle -D FLAG=value format (space after -D)
                            let mut define = parts[i + 1].to_string();
                            // Fix escaped quotes in defines
                            define = define.replace("\\\"", "\""); // Convert \" to "
                            defines.insert(define.clone());
                        }
                    }
                }
            }
        }
    }
    
    // Print a sample of what we found
    println!("cargo:warning=Found {} include paths", include_paths.len());
    let mut include_vec: Vec<String> = include_paths.iter().take(5).cloned().collect();
    println!("cargo:warning=Sample of include paths: {:?}", include_vec);
    
    println!("cargo:warning=Found {} defines", defines.len());
    let mut defines_vec: Vec<String> = defines.iter().take(5).cloned().collect();
    println!("cargo:warning=Sample of defines: {:?}", defines_vec);
    
    // Make sure we have the component's include directory
    let component_include = format!("components/{}/src", component_name);
    if !include_paths.contains(&component_include) {
        println!("cargo:warning=Adding component src directory: {}", component_include);
        include_paths.insert(component_include);
    }

    println!("cargo:warning=Found {} include paths and {} defines for component {}", 
        include_paths.len(), defines.len(), component_name);
    
    // Print the first few defines to help with debugging
    let mut defines_vec: Vec<String> = defines.into_iter().collect();
    if !defines_vec.is_empty() {
        defines_vec.sort();
        println!("cargo:warning=Sample of defines: {:?}", 
            &defines_vec.iter().take(5).collect::<Vec<_>>());
    }
    
    (include_paths.into_iter().collect(), defines_vec)
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
    
    // Get include paths and defines from compile_commands.json for the component
    let (include_paths, defines) = extract_include_paths(component_name);
    
    // Convert include paths to clang args
    let include_args: Vec<String> = include_paths
        .into_iter()
        .map(|path| format!("-I{}", path))
        .collect();

    // Convert defines to clang args
    let define_args: Vec<String> = defines
        .into_iter()
        .map(|def| format!("-D{}", def))
        .collect();

    // Start with basic args for C++ compilation
    let mut clang_args = vec![
        "-x".to_string(), 
        "c++".to_string(), 
        "-std=c++14".to_string(),
        "--target=x86_64-linux".to_string(),
    ];
    
    // Add the rest of the include paths and defines
    clang_args.extend(include_args);
    clang_args.extend(define_args);
    
    // Print actual command for debugging
    println!("cargo:warning=Actual command: clang {} -E -", clang_args.join(" "));
    
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

/// Discover ESP-IDF component include directories


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
