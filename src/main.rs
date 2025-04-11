use std::thread;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp32_blink_idf::lx16_aservo::ffi::{new_bus, new_servo};

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF
    esp_idf_svc::sys::link_patches();
    
    // Initialize logger
    EspLogger::initialize_default();
    log::info!("Starting servo movement example...");
    
    // Get access to the ESP32 peripherals
    let _peripherals = Peripherals::take()?;
    
    // Define the TX and RX pins for the servo bus
    // Note: Use appropriate pin numbers for your hardware setup
    let tx_pin: u8 = 17;  // GPIO 17 for TX
    let rx_pin: u8 = 16;  // GPIO 16 for RX
    
    // Create a new servo bus
    log::info!("Creating servo bus on TX pin {} and RX pin {}", tx_pin, rx_pin);
    let bus = new_bus(tx_pin, rx_pin);
    
    // Create a new servo with ID 1
    let servo_id: u8 = 1;
    log::info!("Creating servo with ID {}", servo_id);
    let servo = new_servo(&bus, servo_id);
    
    // Initialize the servo
    log::info!("Initializing servo...");
    servo.initialize();
    
    // Set servo position limits (in 0.1 degrees)
    // For example, from 0 to 240 degrees would be 0 to 2400 in centidegrees
    servo.setLimitsTicks(0, 2400);
    
    log::info!("Starting servo movement loop...");
    let mut counter = 0;
    
    // Move the servo back and forth in a loop
    loop {
        // Move to minimum position (0 degrees)
        log::info!("Moving to minimum position - iteration {}", counter);
        servo.move_time(0, 1000);  // Move to 0 degrees over 1 second
        thread::sleep(Duration::from_secs(2));  // Wait for movement to complete
        
        // Move to middle position (120 degrees)
        log::info!("Moving to middle position - iteration {}", counter);
        servo.move_time(1200, 1000);  // Move to 120 degrees over 1 second
        thread::sleep(Duration::from_secs(2));  // Wait for movement to complete
        
        // Move to maximum position (240 degrees)
        log::info!("Moving to maximum position - iteration {}", counter);
        servo.move_time(2400, 1000);  // Move to 240 degrees over 1 second
        thread::sleep(Duration::from_secs(2));  // Wait for movement to complete
        
        // Increment counter
        counter += 1;
        
        log::info!("Completed movement cycle {}", counter);
    }
    
    // This line is never reached due to the infinite loop
    #[allow(unreachable_code)]
    Ok(())
}
