pub mod main_extern;
// Import the auto-generated bindings
mod lx16a_servo;

use log::{info, error};
use esp_idf_hal::gpio::{Gpio2, PinDriver, Output};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::log::EspLogger;
use main_extern::hello;
use std::ptr;
use std::mem;

// Use the bindings from the auto-generated file
use lx16a_servo::root::{LX16ABus, LX16AServo, HardwareSerial};

// Main entry point called from C
#[no_mangle]
pub extern "C" fn rust_main() {
    // Initialize the ESP logger
    EspLogger::initialize_default();
    
    info!("Rust main function started");
    
    // Initialize LX16A servo components
    info!("Initializing servo components");
    
    // Create the LX16ABus instance
    // Safety: We're calling into C++ code but managing the lifetime in Rust
    let mut bus = unsafe {
        let bus = LX16ABus::new();
        info!("Successfully created servo bus");
        bus
    };
    
    // Create a servo on the bus with ID 1
    // Safety: We're working with raw pointers to the bus and creating a servo instance
    let mut servo = unsafe {
        // Get a raw pointer to the bus
        let bus_ptr: *mut LX16ABus = &mut bus;
        
        // Create the servo with ID 1
        let mut servo = LX16AServo::new(bus_ptr, 1);
        info!("Successfully created servo with ID 1");
        
        // Initialize the servo
        servo.initialize();
        servo
    };
    
    // Initialize GPIO for the LED
    let mut led = PinDriver::output(unsafe { Gpio2::new() }).unwrap();
    let mut led_state = false;
    
    // Main loop
    info!("Entering main loop");
    loop {
        // Toggle the LED
        led_state = !led_state;
        if led_state {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
        
        // Call our C function and move the servo with proper unsafe boundaries
        unsafe {
            // Call our C hello function
            let _ = hello(4);
            
            // Move the servo - using direct FFI calls to the generated bindings
            if led_state {
                // Move to 50 degrees (5000 centidegrees) over 1 second (1000ms)
                servo.move_time(5000, 1000);
                info!("Moved servo to 50 degrees");
            } else {
                // Move to 0 degrees over 1 second
                servo.move_time(0, 1000);
                info!("Moved servo to 0 degrees");
            }
        }
        
        // Delay for 2 seconds
        FreeRtos::delay_ms(2000);
    }
}
