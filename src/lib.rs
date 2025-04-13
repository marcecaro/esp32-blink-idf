pub mod lx16_aservo;
pub mod main_extern;

use log::info;
use esp_idf_hal::gpio::{Gpio2, PinDriver, Output};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::log::EspLogger;
use main_extern::hello;

// Main entry point called from C
#[no_mangle]
pub extern "C" fn rust_main() {
    // Initialize the ESP logger
    EspLogger::initialize_default();
    
    info!("Rust main function started");
    
    // Set up the GPIO pin for the LED (usually GPIO2 on most ESP32 dev boards)
    let led = unsafe { Gpio2::new() };
    let mut led_driver = PinDriver::output(led).unwrap();
    
    info!("LED pin configured, starting blink loop");
    
    // Main application loop that blinks the LED
    let mut count = 0;
    loop {
        // Toggle the LED
        led_driver.set_high().unwrap();
        info!("LED ON (iteration {})", count);
        
        // Wait 500ms
        FreeRtos::delay_ms(500);
        
        // Toggle the LED
        led_driver.set_low().unwrap();
        info!("LED OFF (iteration {})", unsafe { hello(count) });
        
        // Wait 500ms
        FreeRtos::delay_ms(500);
        
        // Increment the counter
        count += 1;
    }
}
