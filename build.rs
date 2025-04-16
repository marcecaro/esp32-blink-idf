use std::env;
use std::path::PathBuf;
use std::fs;

/// Find system include paths for the specified toolchain
/// 
/// This function searches for system include paths containing important headers like machine/endian.h
/// 
/// Returns a vector of -I flags for include paths
fn get_system_include_paths() -> Vec<String> {
    let includes = fs::read_to_string(".system-includes.txt").unwrap().lines().map(|s| format!("-I{}", s)).collect();
    includes
}

fn get_system_flags() -> Vec<String> {
    let flags = fs::read_to_string(".system-flags.txt").unwrap().lines().map(|s| format!("{}", s)).collect();
    flags
}


/// Generate bindings for the lx16a-servo component
fn generate_bindings(component_name: String) {
    println!("cargo:warning=Generating bindings for {}", component_name);
    
    // Get output directories
    let project_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let component_path = project_path.join("components").join(&component_name);
    let output_mod_path = project_path.join("src");
    let output_mod_path_file = output_mod_path.join(&format!("{}.rs", component_name.replace("-", "_")));

    
    // Build bindgen configuration with minimal flags
    let mut clang_args = vec![
        "--target=x86_64-unknown-linux-gnu".to_string(),
        "-D__XTENSA__".to_string(),
        "-x".to_string(),
        "c++".to_string(),
        "-std=c++11".to_string(),
        "-nostdinc".to_string(),
        "-nostdinc++".to_string(),
    ];
    

    // Add machine include directory to find machine/endian.h
    let xtensa_includes = get_system_include_paths();
    clang_args.extend(xtensa_includes);

    let system_flags = get_system_flags();
    clang_args.extend(system_flags);
    
    // for arg in &clang_args {
    //     println!("cargo:warning=Using clang arg: {}", arg);
    // }

    let ffi_name = component_path.join("src/ffi.h");   

   if !ffi_name.exists() {
        println!("cargo:warning=FFI header does not exist in {}", ffi_name.to_str().unwrap());
        return;
    }
   
    
    let builder = bindgen::Builder::default()
        .header(ffi_name.to_str().unwrap())
        .enable_cxx_namespaces()
        .allowlist_type("LX16AServo")
        .allowlist_type("LX16ABus")
        // Disable layout tests to avoid size validation issues
        .layout_tests(false)
        .derive_debug(true)
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(clang_args);
    
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
    match bindings.write_to_file(output_mod_path_file) {
        Ok(_) => println!("cargo:warning=Successfully wrote bindings to file"),
        Err(e) => {
            println!("cargo:warning=Couldn't write bindings: {:?}", e);
            panic!("Failed to write bindings: {:?}", e);
        }
    };
        
    // Tell cargo to link with the component library
}

fn main() {
    // Tell cargo to rerun on changes
    println!("cargo:rerun-if-env-changed=ESP_IDF_PATH");
    println!("cargo:rerun-if-env-changed=IDF_PATH");
    
    // Generate bindings
    generate_bindings("lx16a-servo".to_string());
    
    // Link with stdc++
    println!("cargo:rustc-link-lib=stdc++");
    
    // Remove the RSP file reference as it might be causing issues
    // println!("cargo:rustc-link-arg=@build/esp-idf/lx16a-servo/liblx16a-servo.a.rsp");
}
