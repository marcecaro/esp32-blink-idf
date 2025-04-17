use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

// Import our lx16a module
mod lx16a;
use esp_idf_svc::log::EspLogger;
use log::*;
use esp_idf_hal::delay::FreeRtos;
use crate::lx16a::*;
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    
    // Initialize the ESP logger
    EspLogger::initialize_default();

    info!("ESP32 LX16A Servo Example Starting...");
    
    // Get Serial1 for servo communication
    let serial = unsafe { lx16a::getSerial1() };
    
    // Create the servo bus - equivalent to servoBus.begin(&Serial1, 1, 2)
    let servo_bus = ServoBus::new(serial, 1, 2);
    servo_bus.debug(true);
    
    // Create the servo - equivalent to LX16AServo servo(&servoBus, 1)
    let servo = Servo::new(&servo_bus, 1);
    
    // Main application loop
    info!("Beginning Servo Example");
    
    loop {
        let divisor = 4;
        
        for i in 0..1000/divisor {
            let start = Instant::now();
            
            let angle = i * 24 * divisor;
            
            // Read current position
            let pos = servo.pos_read();
            info!("\n\nPosition at {} -> {}", pos, 
                  if servo.is_command_ok() { "OK" } else { "\n\nERR!!\n\n" });
            
            // Keep trying until the command is successful
            loop {
                servo.move_time(angle, 10 * divisor as u16);
                if servo.is_command_ok() { break; }
            }
            
            info!("Move to {} -> {}", angle, 
                  if servo.is_command_ok() { "OK" } else { "\n\nERR!!\n\n" });
            
            info!("Voltage = {}", servo.vin());
            info!("Temp = {}", servo.temperature());
            info!("ID = {}", servo.id_read());
            info!("Motor Mode = {}", servo.read_is_motor_mode());
            
            // Calculate how long this iteration took and delay accordingly
            let elapsed = start.elapsed();
            let target_duration = Duration::from_millis((10 * divisor) as u64);
            
            if elapsed < target_duration {
                let delay_time = target_duration - elapsed;
                FreeRtos::delay_ms(delay_time.as_millis() as u32);
            } else {
                info!("Real Time broken, took: {}ms", elapsed.as_millis());
            }
        }
        
        // Return to starting position
        servo.move_time(0, 3000);
        FreeRtos::delay_ms(3000);
    }
}
