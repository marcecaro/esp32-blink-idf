use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

// Using regular code with no special debug markers

// Import our lx16a module
mod lx16a;
use esp_idf_svc::log::EspLogger;
use log::*;
use esp_idf_hal::delay::FreeRtos;
use crate::lx16a::*;
use std::time::{Duration, Instant};

// main.rs (or main.cpp if you prefer C++)
unsafe extern "C" {
    unsafe fn initArduino();
}

fn arduino_init() {
    unsafe { initArduino() };      // sets up RTOS hooks, heap, USB‑CDC, etc.
}


fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF patches
    esp_idf_sys::link_patches();
    // Simple set of variables to make breakpoints more reliable
    
    // Give debugger time to attach
    
    info!("Initializing Arduino...");
    arduino_init();
    info!("Arduino initialized");
    // Set ESP-IDF logger
    EspLogger::initialize_default();
    info!("ESP-IDF Logger initialized");
    

    warn!("ESP32 LX16A Servo Example Starting...");
    
    // Get Serial1 for servo communication
    // Create the servo bus - equivalent to servoBus.begin(&Serial1, 1, 2)
    let servo_bus = ServoBus::new();
    servo_bus.begin_one_pin_mode(LX16AHardwareSerial::new_2(), 33);
    servo_bus.debug(true);
    
    // Create the servo - equivalent to LX16AServo servo(&servoBus, 1)
    let servo = Servo::new(&servo_bus, 1);
    servo.initialize(); // Servo Number is 1
    
    // Main application loop
    info!("Main loop starting");
    
    warn!("System fully initialized");
    loop {
        #[cfg(debug_assertions)]
        {
            // This line is for GDBStub debugging, not needed for JTAG
            // debug_break();  // This will trigger the GDBStub
        }
        let divisor = 4;
        
        // Good breakpoint spot here
        
        for i in 0..1000/divisor {
            let start = Instant::now();
            
            let angle = i * 24 * divisor;
            
            // Read current position
            // let pos = servo.pos_read();
            // info!("\n\nPosition at {} -> {}", pos, 
            //       if servo.is_command_ok() { "OK" } else { "\n\nERR!!\n\n" });
            
            // Keep trying until the command is successful
           // loop {
                servo.move_time(angle, 10 * divisor as u16);
            //    if servo.is_command_ok() { break; }
            //}
            
            
            warn!("Voltage = {}", servo.vin());
            warn!("Temp = {}", servo.temperature());
            warn!("ID = {}", servo.id_read());
            warn!("Motor Mode = {}", servo.read_is_motor_mode());
            
            // Calculate how long this iteration took and delay accordingly
            let elapsed = start.elapsed();
            let target_duration = Duration::from_millis((10 * divisor) as u64);
            
            info!("Iteration complete in {}ms", elapsed.as_millis());
            
            if elapsed < target_duration {
                let delay_time = target_duration - elapsed;
                info!("Delaying for {}ms", delay_time.as_millis());
                FreeRtos::delay_ms(delay_time.as_millis() as u32);
            } else {
                warn!("Real Time broken, took: {}ms", elapsed.as_millis());
            }
        }
        // let the scheduler breathe for 1 tick (important for stability!)
        info!("Yielding to scheduler");
        unsafe { esp_idf_sys::vTaskDelay(1) };
        info!("Scheduler yielded");
        // Return to starting position
        servo.move_time(0, 100);
        FreeRtos::delay_ms(500);
    }
}
