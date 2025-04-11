use std::thread;
use std::time::Duration;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp32_blink_idf::lx16_aservo::ffi::hello;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    hello(1);
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
    
    loop {
        // Toggle LED state

        led.toggle()?;
        log::info!("LED ON - Loop iteration: {}", counter);
        
        // Increment counter
        counter += 1;
        
        // Delay for 1 second
        thread::sleep(Duration::from_secs(1));
    }
    
    // This line is never reached due to the infinite loop
    #[allow(unreachable_code)]
    Ok(())
}
