use bindgen;
use bindgen::callbacks::ParseCallbacks;
use std::path::PathBuf;
use std::fs;

// Custom callback implementation for bindgen
#[derive(Debug)]
struct CargoCallbacks;

impl CargoCallbacks {
    fn new() -> Self {
        CargoCallbacks {}
    }
}

impl ParseCallbacks for CargoCallbacks {}

fn main() {
    // Relay and output ESP-IDF link flags to the Rust compiler
    embuild::espidf::sysenv::output();

    let headers = [("components/lx16a-servo/src/lx16a_c_wrapper.h", "lx16a")];

    for (header, mod_name) in headers {
        // Make sure the output directory exists
        let out_dir = PathBuf::from("src").join(mod_name);
        fs::create_dir_all(&out_dir).ok();
        

        // Generate the bindings
        let bindings = bindgen::Builder::default()
            .header(header)
            .clang_arg("--target=x86_64-unknown-linux-gnu")
            
            // Derive common traits where possible
            .derive_copy(true)
            .derive_debug(true)
            .derive_default(true)
            .derive_hash(true)
            .derive_eq(true)
            .derive_ord(true)
            
            // Implement traits for types that can't derive them
            .impl_debug(true)
            .impl_partialeq(true)
            
            // Generate wrappers for inline functions
            .generate_inline_functions(true)
            
            // Use a custom parse callback
            .parse_callbacks(Box::new(CargoCallbacks::new()))
            
            .generate()
            .expect("Unable to generate bindings");

        bindings.write_to_file(&out_dir.join("ffi.rs"))
            .expect("Couldn't write bindings!");
    }

}