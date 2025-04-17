use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_svc::log::EspLogger;
use log::*;
use esp_idf_hal::delay::FreeRtos;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    
    // Initialize the ESP logger
    EspLogger::initialize_default();

    info!("ESP32 Hello World Application Starting...");

    // Main application loop
    loop {
        info!("Hello, World from ESP32!");
        
        // Delay for 1 second
        FreeRtos::delay_ms(1000);
    }
}
