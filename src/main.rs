use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_svc::log::EspLogger;
use log::*;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::uart::config::{Config, DataBits, StopBits};
use esp_idf_hal::units::Hertz;

// Import lewan_bus module
mod lewan_bus;
use crate::lewan_bus::LewanSoulBus;
// main.rs (or main.cpp if you prefer C++)

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF patches
    esp_idf_sys::link_patches();
    
    // Initialize the ESP logger
    EspLogger::initialize_default();
    info!("ESP-IDF Rust initialized");
        // Take all peripherals
    let peripherals = Peripherals::take().unwrap();

    // UART1 with default pins (TX=GPIO32, RX=GPIO33)
    let config = Config::default().baudrate(Hertz(115_200))
    .data_bits(DataBits::DataBits8)
    .parity_none()
    .stop_bits(StopBits::STOP1);
    
    let mut bus = LewanSoulBus::new(
        peripherals.uart1,
        peripherals.pins.gpio32,
        peripherals.pins.gpio33,
        &config,
    )?;

    // Set servo ID 1 to 90 degrees in 1 second
    

    // Main loop that runs indefinitely
    let mut i = 0;  
    loop {
        // Move servo to 90 degrees

        // Read current position
        match bus.read_position(1) {
            Ok(pos) => println!("Servo position (0-1000 units): {}", pos),
            Err(e) => error!("Failed to read position: {:?}", e),
        }
        // println!("Moving servo ID 1 to {}°...", i%240); 
        bus.move_to_position(1, i%1000, 1000).expect("Cannot move!");
        i += 403;

        // println!("Moving servo ID 1 to {}°...", i%240); 
        bus.move_to_position(2, (i+345)%1000, 1000).expect("Cannot move!");
        i += 403;

        // Give it some time to move
        sleep(Duration::from_millis(2000));

        // // Read current position
        // match bus.read_position(1) {
        //     Ok(pos) => info!("Servo position (0-1000 units): {}", pos),
        //     Err(e) => error!("Failed to read position: {:?}", e),
        // }

        // // Enable torque (power)
        // bus.set_torque(1, true)?;

        // // Set angle limits (45° to 135°)
        // bus.set_angle_limits(1, 45.0, 135.0)?;

        // // Switch to motor mode at half speed forward
        // //bus.set_mode(1, true, 500)?;

        // wait 500 ms
        //sleep(Duration::from_millis(500));
        
        // To exit the loop (we won't reach this in practice)
        // Uncomment this to make the program exit after one cycle
        // if condition_to_exit {
        //     break Ok(());
        // }
    }
}
