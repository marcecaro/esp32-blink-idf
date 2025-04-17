pub mod main_extern;
// Import the auto-generated bindings
mod lx16a_servo;

// Minimize dependencies to prevent standard library issues
use log::info;
use esp_idf_hal::gpio::{Gpio2, PinDriver};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::log::EspLogger;
use main_extern::hello;

// Use the bindings from the auto-generated file
use lx16a_servo::root::{LX16ABus, LX16AServo};

// Tell Rust not to insert its own main function
#[allow(unused_attributes)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::not_unsafe_ptr_arg_deref))]

// Main entry point called from C
#[no_mangle]
pub extern "C" fn rust_main() {
    // Initialize the ESP logger
    EspLogger::initialize_default();
    
    info!("Rust main function started");
    
    // Initialize GPIO for the LED
    let led = match PinDriver::output(unsafe { Gpio2::new() }) {
        Ok(led) => led,
        Err(_) => {
            info!("Failed to initialize LED");
            return;
        }
    };
    
    // Only test basic FFI function to confirm linking works
    let hello_result = unsafe { hello(4) };
    info!("Called C function hello with result: {}", hello_result);
    
    // Avoid the full LX16A implementation for now - just test symbol resolution
    test_servo_symbols();
}

// Function to test LX16A symbol resolution without executing full code
#[inline(never)]
fn test_servo_symbols() {
    // This function only serves to ensure the symbols are linked
    // without running the actual code (to avoid hardware dependencies)
    info!("Testing servo symbol resolution...");
    
    // Create the bus and servo instance pointers
    // The compiler should force these symbols to be included, but we won't actually use them
    unsafe {
        // Get the function pointers but don't call them - just ensure they're linked
        let bus_constructor = LX16ABus::new as *const ();
        let servo_constructor = LX16AServo::new as *const ();
        let initialize_fn = LX16AServo::initialize as *const ();
        let move_time_fn = LX16AServo::move_time as *const ();
        
        // Print the addresses to make sure the compiler doesn't optimize away
        info!("Bus constructor: {:p}", bus_constructor);
        info!("Servo constructor: {:p}", servo_constructor);
        info!("Initialize method: {:p}", initialize_fn);
        info!("Move time method: {:p}", move_time_fn);
    }
    
    info!("Symbol resolution test completed");
}
