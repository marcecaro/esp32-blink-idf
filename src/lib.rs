pub mod lx16_aservo;
pub mod main_extern;

use log::{info, error};
use esp_idf_hal::gpio::{Gpio2, PinDriver, Output};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::log::EspLogger;
use main_extern::hello;

// Import our safe wrapper types
use lx16_aservo::{Bus, Servo};

// Main entry point called from C
#[no_mangle]
pub extern "C" fn rust_main() {
    // Initialize the ESP logger
    EspLogger::initialize_default();
    
    info!("Rust main function started");
    
    // Initialize LX16A servo components
    info!("Initializing servo components");
    
    // Create the bus and servo with safe Rust wrappers
    // These handle all the unsafe FFI calls internally and provide proper error handling
    let bus = match Bus::new(1, 2) {
        Ok(bus) => {
            info!("Successfully created servo bus");
            bus
        },
        Err(e) => {
            error!("Failed to create servo bus: {:?}", e);
            return;
        }
    };
    
    // Enable debug output
    bus.debug(true);
    
    // Create and initialize the servo
    let servo = match bus.create_servo(1) {
        Ok(servo) => {
            info!("Successfully created servo");
            servo
        },
        Err(e) => {
            error!("Failed to create servo: {:?}", e);
            return;
        }
    };
    
    // Enable the servo and move to initial position
    servo.enable();
    if let Err(e) = servo.move_time(500, 1000) { // Move to position 500 over 1 second
        error!("Failed to move servo: {:?}", e);
    }
    
    info!("Servo components initialized");
    
    // Set up the GPIO pin for the LED (usually GPIO2 on most ESP32 dev boards)
    let led = unsafe { Gpio2::new() };
    let mut led_driver = PinDriver::output(led).unwrap();
    
    info!("LED pin configured, starting main loop");
    
    // Main application loop that blinks the LED and controls the servo
    let mut count = 0;
    let mut servo_angle = 0;
    let divisor = 4;
    
    loop {
        // Toggle the LED
        led_driver.set_high().unwrap();
        info!("LED ON (iteration {})", count);
        
        // Read the current servo position with proper error handling
        match servo.pos_read() {
            Ok(pos) => info!("Servo position: {}", pos),
            Err(e) => error!("Failed to read servo position: {:?}", e)
        };
        
        // Move the servo to a new position with proper error handling
        if let Err(e) = servo.move_time(servo_angle, 10 * divisor as u16) {
            error!("Failed to move servo: {:?}", e);
        } else {
            info!("Moving servo to angle: {} (time: {}ms)", servo_angle, 10 * divisor);
        }
        
        // Read and display servo information with proper error handling
        if let Ok(voltage) = servo.vin() {
            info!("Servo voltage: {}", voltage);
        }
        
        if let Ok(temp) = servo.temp() {
            info!("Servo temperature: {}", temp);
        }
        
        if let Ok(id) = servo.id_read() {
            info!("Servo ID: {}", id);
        }
        
        if let Ok(is_motor_mode) = servo.is_motor_mode() {
            info!("Servo motor mode: {}", is_motor_mode);
        }
        
        // Wait for the movement to complete
        FreeRtos::delay_ms(10 * divisor as u32);
        
        // Toggle the LED
        led_driver.set_low().unwrap();
        info!("LED OFF (iteration {})", unsafe { hello(count) });
        
        // Wait another half second
        FreeRtos::delay_ms(500);
        
        // Update angle for next iteration (0-24000 in steps, similar to C++ example)
        servo_angle = (servo_angle + 24 * divisor) % 24000;
        
        // Every 1000 iterations, move servo back to 0
        if count % 40 == 39 {
            info!("Resetting servo to position 0");
            unsafe { 
                // Store the servo pointer for later use if needed
                lx16_aservo::SERVO_PTR = servo.as_ptr();
                // Call the C++ function directly
                lx16_aservo::LX16AServo_move_time(lx16_aservo::SERVO_PTR, 0, 3000);
            }
            FreeRtos::delay_ms(3000);
            servo_angle = 0;
        }
        
        // Increment the counter
        count += 1;
    }
    
    // Resources will be automatically cleaned up when they go out of scope
    // The Bus and Servo structs implement Drop trait to safely free C++ resources
    // This code is never reached due to the infinite loop, but is good practice
}
