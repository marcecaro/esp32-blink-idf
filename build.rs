// build.rs
// Set up ESP-IDF environment for Rust binary project
// following the Rust as main entrypoint approach

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to rebuild if these files change
    embuild::espidf::sysenv::output();
    
    Ok(())
}