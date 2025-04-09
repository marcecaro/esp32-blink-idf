// build.rs for ESP32 project with C++ components
use std::process::Command;
use std::path::Path;

fn main() {
    // Print information about the build
    println!("cargo:warning=Building ESP32 project with C++ components");

    // Propagate ESP-IDF configuration
    embuild::build::CfgArgs::output_propagated("ESP_IDF").unwrap();
    embuild::build::LinkArgs::output_propagated("ESP_IDF").unwrap();

    // Source the ESP-IDF environment script
    if Path::new("./project-export.sh").exists() {
        // Don't call idf.py build from within build.rs to avoid circular dependency
        let status = Command::new("bash")
            .arg("-c")
            .arg(". ./project-export.sh")
            .status()
            .expect("Failed to execute ESP-IDF environment setup");
            
        if !status.success() {
            panic!("ESP-IDF environment setup failed");
        }

        println!("cargo:warning=ESP-IDF environment setup successful");
    } else {
        println!("cargo:warning=project-export.sh not found, skipping ESP-IDF environment setup");
    }
    
    // Watch for changes in component files
    println!("cargo:rerun-if-changed=components/esp32servoserver/CMakeLists.txt");
    println!("cargo:rerun-if-changed=components/esp32servoserver/src");
}