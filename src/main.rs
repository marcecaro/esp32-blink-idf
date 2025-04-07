use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;                     // for .kHz() method
use esp_idf_hal::i2c::{I2cDriver, I2cConfig};    // ESP-IDF HAL I2C driver
use esp_idf_hal::delay::FreeRtos;               // FreeRTOS-based delay
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting LED blink loop...");
    
    // Get access to the ESP32 peripherals
    let peripherals = Peripherals::take()?;
    
    // The built-in LED on most ESP32 dev boards is connected to GPIO2
    let led_pin = peripherals.pins.gpio2;
    
    // Create a pin driver for the LED pin in output mode
    let mut led = PinDriver::output(led_pin)?;
    
    // Create an infinite loop
    let mut counter = 0;
    
    // Configure I2C driver at 100 kHz on SDA=GPIO21, SCL=GPIO22
    let i2c = I2cDriver::new(
        peripherals.i2c0, 
        peripherals.pins.gpio21, 
        peripherals.pins.gpio22, 
        &I2cConfig::new().baudrate(100.kHz().into())
    )?;

    // Initialize a 16x2 character LCD via the PCF8574 I2C adapter
    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd16x2, FreeRtos);
    lcd.init().ok();        // power up and default initialize the LCD

    // Optionally, turn on backlight and clear screen (if not already done by init)
    lcd.backlight(true).ok();
    lcd.clear().ok();

    loop {
        // Toggle LED state

        led.toggle()?;
        log::info!("LED ON - Loop iteration: {}", counter);
        
        // Increment counter
        counter += 1;
        

        // Print messages on the LCD
        lcd.set_cursor(0, 0).ok();
        lcd.print("Hello, ESP32!").ok();  // print on first line
        lcd.set_cursor(0, 1).ok();    // move cursor to col 0, second line (row index 1)
        lcd.print(&format!("Rustloop: {}", counter)).ok(); // print on second line
        
        // Delay for 1 second
        thread::sleep(Duration::from_secs(1));
    }
    
    // This line is never reached due to the infinite loop
    #[allow(unreachable_code)]
    Ok(())
}
