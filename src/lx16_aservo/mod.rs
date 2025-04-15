use std::fmt::{self, Debug};
use std::ptr::NonNull;

// Define our FFI interface using cxx bridge
#[cxx::bridge]
pub mod ffi {
    // Opaque types from C++
    unsafe extern "C++" {
        // LX16ABus C++ class (opaque to Rust)
        type LX16ABus;
        
        // LX16AServo C++ class (opaque to Rust)
        type LX16AServo;
        
        // Functions for LX16ABus
        unsafe fn LX16ABus_new() -> *mut LX16ABus;
        unsafe fn LX16ABus_delete(bus: *mut LX16ABus);
        unsafe fn LX16ABus_begin(bus: *mut LX16ABus, port: i32, tx_pin: i32) -> bool;
        unsafe fn LX16ABus_debug(bus: *mut LX16ABus, enable: bool);
        
        // Functions for LX16AServo
        unsafe fn LX16AServo_new(bus: *mut LX16ABus, id: u8) -> *mut LX16AServo;
        unsafe fn LX16AServo_delete(servo: *mut LX16AServo);
        unsafe fn LX16AServo_initialize(servo: *mut LX16AServo) -> bool;
        unsafe fn LX16AServo_move_time(servo: *mut LX16AServo, angle: i32, time_ms: u16) -> bool;
        unsafe fn LX16AServo_pos_read(servo: *mut LX16AServo, position: *mut i32) -> bool;
        unsafe fn LX16AServo_enable(servo: *mut LX16AServo) -> bool;
        unsafe fn LX16AServo_disable(servo: *mut LX16AServo) -> bool;
        unsafe fn LX16AServo_id_read(servo: *mut LX16AServo) -> u8;
        unsafe fn LX16AServo_vin_read(servo: *mut LX16AServo, voltage: *mut i32) -> bool;
        unsafe fn LX16AServo_temp_read(servo: *mut LX16AServo, temp: *mut i32) -> bool;
        unsafe fn LX16AServo_is_motor_mode(servo: *mut LX16AServo, is_motor: *mut bool) -> bool;
    }
}

// Safe Rust wrapper for LX16ABus
pub struct Bus {
    // Raw pointer to the C++ object
    ptr: NonNull<ffi::LX16ABus>,
}

impl Bus {
    // Create a new bus connected to the given serial port and tx pin
    pub fn new(port: i32, tx_pin: i32) -> Result<Self, &'static str> {
        // Create the C++ object
        let ptr = unsafe { ffi::LX16ABus_new() };
        
        // Ensure the pointer is valid
        let ptr = NonNull::new(ptr).ok_or("Failed to create LX16ABus")?
        ;
        
        // Create the Rust wrapper
        let mut bus = Self { ptr };
        
        // Initialize the bus
        if unsafe { ffi::LX16ABus_begin(bus.ptr.as_ptr(), port, tx_pin) } {
            Ok(bus)
        } else {
            Err("Failed to initialize LX16ABus")
        }
    }
    
    // Enable or disable debug output
    pub fn debug(&self, enable: bool) {
        unsafe { ffi::LX16ABus_debug(self.ptr.as_ptr(), enable) };
    }
    
    // Create a new servo on this bus
    pub fn create_servo(&self, id: u8) -> Result<Servo, &'static str> {
        Servo::new(self.ptr.as_ptr(), id)
    }
}

// Clean up C++ resources when the Rust object is dropped
impl Drop for Bus {
    fn drop(&mut self) {
        unsafe { ffi::LX16ABus_delete(self.ptr.as_ptr()) };
    }
}

// Safe debugging implementation
impl Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LX16ABus").finish()
    }
}

// Safe Rust wrapper for LX16AServo
pub struct Servo {
    // Raw pointer to the C++ object
    ptr: NonNull<ffi::LX16AServo>,
}

impl Servo {
    // Create a new servo on the given bus with the specified ID
    pub fn new(bus_ptr: *mut ffi::LX16ABus, id: u8) -> Result<Self, &'static str> {
        // Create the C++ object
        let ptr = unsafe { ffi::LX16AServo_new(bus_ptr, id) };
        
        // Ensure the pointer is valid
        let ptr = NonNull::new(ptr).ok_or("Failed to create LX16AServo")?
        ;
        
        // Create the Rust wrapper
        let servo = Self { ptr };
        
        Ok(servo)
    }
    
    // Initialize the servo
    pub fn initialize(&self) -> Result<(), &'static str> {
        if unsafe { ffi::LX16AServo_initialize(self.ptr.as_ptr()) } {
            Ok(())
        } else {
            Err("Failed to initialize servo")
        }
    }
    
    // Enable the servo
    pub fn enable(&self) -> Result<(), &'static str> {
        if unsafe { ffi::LX16AServo_enable(self.ptr.as_ptr()) } {
            Ok(())
        } else {
            Err("Failed to enable servo")
        }
    }
    
    // Disable the servo
    pub fn disable(&self) -> Result<(), &'static str> {
        if unsafe { ffi::LX16AServo_disable(self.ptr.as_ptr()) } {
            Ok(())
        } else {
            Err("Failed to disable servo")
        }
    }
    
    // Move the servo to a position over time
    pub fn move_time(&self, angle: i32, time_ms: u16) -> Result<(), &'static str> {
        if unsafe { ffi::LX16AServo_move_time(self.ptr.as_ptr(), angle, time_ms) } {
            Ok(())
        } else {
            Err("Failed to move servo")
        }
    }
    
    // Read the current position
    pub fn pos_read(&self) -> Result<i32, &'static str> {
        let mut position: i32 = 0;
        if unsafe { ffi::LX16AServo_pos_read(self.ptr.as_ptr(), &mut position) } {
            Ok(position)
        } else {
            Err("Failed to read servo position")
        }
    }
    
    // Read the servo ID
    pub fn id_read(&self) -> Result<u8, &'static str> {
        Ok(unsafe { ffi::LX16AServo_id_read(self.ptr.as_ptr()) })
    }
    
    // Read the input voltage
    pub fn vin(&self) -> Result<i32, &'static str> {
        let mut voltage: i32 = 0;
        if unsafe { ffi::LX16AServo_vin_read(self.ptr.as_ptr(), &mut voltage) } {
            Ok(voltage)
        } else {
            Err("Failed to read servo voltage")
        }
    }
    
    // Read the temperature
    pub fn temp(&self) -> Result<i32, &'static str> {
        let mut temp: i32 = 0;
        if unsafe { ffi::LX16AServo_temp_read(self.ptr.as_ptr(), &mut temp) } {
            Ok(temp)
        } else {
            Err("Failed to read servo temperature")
        }
    }
    
    // Check if the servo is in motor mode
    pub fn is_motor_mode(&self) -> Result<bool, &'static str> {
        let mut is_motor: bool = false;
        if unsafe { ffi::LX16AServo_is_motor_mode(self.ptr.as_ptr(), &mut is_motor) } {
            Ok(is_motor)
        } else {
            Err("Failed to read servo motor mode")
        }
    }
    
    // Get the raw pointer for use in unsafe code
    pub unsafe fn as_ptr(&self) -> *mut ffi::LX16AServo {
        self.ptr.as_ptr()
    }
}

// Clean up C++ resources when the Rust object is dropped
impl Drop for Servo {
    fn drop(&mut self) {
        unsafe { ffi::LX16AServo_delete(self.ptr.as_ptr()) };
    }
}

// Safe debugging implementation
impl Debug for Servo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LX16AServo").finish()
    }
}

// Export the raw C++ function for the unsafe code in lib.rs
pub use ffi::LX16AServo_move_time;

// Public re-export for the servo pointer
pub static mut SERVO_PTR: *mut ffi::LX16AServo = std::ptr::null_mut();
