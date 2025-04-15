use std::env;
use std::path::PathBuf;

/// Find compiler arguments for a component in compile_commands.json
/// 
/// Parameters:
/// - component: The component name (e.g., "lx16a-servo")
/// - ban_dist: List of compile switches to filter out
/// - c_ccp_file: The specific C/C++ file to look for in the commands
/// 
/// Returns a vector of compiler flags suitable for bindgen
fn find_component_include_paths(component: String, ban_dist: Vec<String>, c_ccp_file: String) -> Vec<String> {
    use std::fs;
    let compile_commands_path = "build/compile_commands.json";
    let mut args = Vec::new();
    
    // Read the compile_commands.json file
    let content = match fs::read_to_string(compile_commands_path) {
        Ok(c) => c,
        Err(e) => {
            println!("cargo:warning=Failed to read {}: {}", compile_commands_path, e);
            return args;
        }
    };
    
    // Parse JSON
    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(e) => {
            println!("cargo:warning=Failed to parse compile_commands.json: {}", e);
            return args;
        }
    };
    
    let comp_str = format!("components/{}", component);
    
    if let Some(entries) = json.as_array() {
        // Iterate and pick the first matching command entry
        for entry in entries {
            if let (Some(file_val), Some(cmd_val)) = (entry.get("file"), entry.get("command")) {
                let file = file_val.as_str().unwrap_or("");
                if file.contains(&comp_str) && file.contains(&c_ccp_file) {
                    let cmd = cmd_val.as_str().unwrap_or("");
                    // Tokenize and filter tokens
                    let tokens: Vec<&str> = cmd.split_whitespace().collect();
                    // Skip the first token (the compiler)
                    for token in tokens.iter().skip(1) {
                        if *token == "-o" { continue; }
                        if ban_dist.iter().any(|ban| token.contains(ban)) { continue; }
                        // Fix double-escaped quotes and backslashes
                        let fixed_token = token.replace("\\\\\"", "\"").replace("\\\\", "\\");
                        args.push(fixed_token);
                    }
                    break;
                }
            }
        }
    } else {
        println!("cargo:warning=compile_commands.json is not an array");
    }
    args
}



/// Generate bindings for the lx16a-servo component
fn generate_bindings() {
    println!("cargo:warning=Generating bindings for lx16a-servo");
    
    // Get output directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Define banned compiler flags that cause issues with bindgen
    let banned_flags = vec![
        "-mlongcalls".to_string(),
        "-fstrict-volatile-bitfields".to_string(),
        "-fno-tree-switch-conversion".to_string(),
        "-fno-shrink-wrap".to_string(),
        "-Wno-error".to_string(),
        "-Wall".to_string(),
        "-Werror".to_string(),
        "-Wextra".to_string(),
    ];
    
    // Get component flags from the compile_commands.json
    let component_flags = find_component_include_paths(
        "lx16a-servo".to_string(),
        banned_flags,
        "lx16a-servo.cpp".to_string()
    );
    // Build bindgen configuration with minimal flags
    let mut clang_args = vec![
        "--target=x86_64-unknown-linux-gnu".to_string()
    ];

    // Add source directory explicitly to ensure header can be found
    //clang_args.push(format!("-I{}", std::path::Path::new("components/lx16a-servo/src").to_string_lossy()));
    // Add ESP-IDF include directories to clang args
    clang_args.extend(component_flags);
    
    println!("cargo:warning=Using clang args: {:?}", clang_args);
    
    let builder = bindgen::Builder::default()
        .header("components/lx16a-servo/src/lx16a-servo.h")
        .clang_args(&clang_args)
        .allowlist_type("LX16AServo")
        .opaque_type("HardwareSerial")
        .ctypes_prefix("cty")
        .layout_tests(false)
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    
    println!("cargo:warning=Generating bindings with simplified header");
    let bindings = match builder.generate() {
        Ok(bindings) => {
            println!("cargo:warning=Bindings generated successfully");
            bindings
        },
        Err(e) => {
            println!("cargo:warning=Failed to generate bindings: {:?}", e);
            panic!("Bindgen error: {:?}", e);
        }
    };

    // Write bindings to file
    println!("cargo:warning=Writing bindings to file");
    match bindings.write_to_file(out_path.join("lx16a-servo.rs")) {
        Ok(_) => println!("cargo:warning=Successfully wrote bindings to file"),
        Err(e) => {
            println!("cargo:warning=Couldn't write bindings: {:?}", e);
            panic!("Failed to write bindings: {:?}", e);
        }
    };
        
    // Tell cargo to link with the component library
    println!("cargo:rustc-link-search=native=components/lx16a-servo/build");
    println!("cargo:rustc-link-lib=static=lx16a-servo");
}

fn main() {
    // Tell cargo to rerun on changes
    println!("cargo:rerun-if-changed=build/compile_commands.json");
    println!("cargo:rerun-if-changed=components/lx16a-servo/src/lx16a-servo.h");
    println!("cargo:rerun-if-env-changed=ESP_IDF_PATH");
    println!("cargo:rerun-if-env-changed=IDF_PATH");
    
    // Generate bindings
    generate_bindings();
    
    // Link with stdc++
    println!("cargo:rustc-link-lib=stdc++");
    
    // Add RSP file support for complex linking
    println!("cargo:rustc-link-arg=@build/esp-idf/lx16a-servo/liblx16a-servo.a.rsp");
}
