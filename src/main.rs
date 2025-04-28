use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_svc::log::EspLogger;
use log::*;
use std::thread::sleep;
use std::time::Duration;
use esp_idf_hal::peripherals::Peripherals;



// Import lewan_bus module
mod lewan_bus;
use crate::lewan_bus::LewanSoulBus;

// Import wifi module
mod wifi;
use crate::wifi::wifi_init;

mod servos;
use crate::servos::init_servos;

// Import EspWifi
use esp_idf_svc::wifi::EspWifi;

fn main() -> anyhow::Result<()> {
    // Initialize ESP-IDF patches
    esp_idf_sys::link_patches();
    
    // Initialize the ESP logger
    EspLogger::initialize_default();
    info!("ESP-IDF Rust initialized");
        // Take all peripherals
    let peripherals = Peripherals::take().unwrap();

    
    // Set servo ID 1 to 90 degrees in 1 second
    let _wifi: EspWifi = wifi_init("fliacaro", "50344212")?;

    let mut bus: LewanSoulBus = init_servos(peripherals.uart1, peripherals.pins.gpio32, peripherals.pins.gpio33)?;

    // Main loop that runs indefinitely
    let mut i = 0;  
    loop {
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

    }
}
