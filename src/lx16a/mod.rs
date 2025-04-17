
// Re-export the FFI bindings
#[allow(warnings)]
pub mod ffi;
pub use ffi::*;

// A safe wrapper around the LX16ABus pointer
pub struct ServoBus {
    ptr: *mut LX16ABusHandle,
}

impl ServoBus {
    pub fn new(serial: *mut HardwareSerial, tx_pin: i32, tx_flag_gpio: i32) -> Self {
        let ptr = unsafe { lx16a_bus_create(serial, 115200, tx_pin, tx_flag_gpio) };
        Self { ptr }
    }

    pub fn debug(&self, enable: bool) {
        unsafe { lx16a_bus_debug(self.ptr, enable) };
    }
}

impl Drop for ServoBus {
    fn drop(&mut self) {
        unsafe { lx16a_bus_destroy(self.ptr) };
    }
}

// A safe wrapper around the LX16AServo pointer
pub struct Servo {
    ptr: *mut LX16AServoHandle,
}

impl Servo {
    pub fn new(bus: &ServoBus, id: u8) -> Self {
        let ptr = unsafe { lx16a_servo_create(bus.ptr, id) };
        Self { ptr }
    }

    pub fn move_time(&self, angle: i32, time_ms: u16) {
        unsafe { lx16a_servo_move_time(self.ptr, angle, time_ms) };
    }

    pub fn pos_read(&self) -> i32 {
        unsafe { lx16a_servo_pos_read(self.ptr) }
    }

    pub fn temperature(&self) -> i32 {
        unsafe { lx16a_servo_temperature(self.ptr) }
    }

    pub fn is_command_ok(&self) -> bool {
        unsafe { lx16a_servo_cmd_ok(self.ptr) }
    }

    #[allow(dead_code)]
    pub fn set_id(&self, new_id: u8) {
        unsafe { lx16a_servo_set_id(self.ptr, new_id) };
    }

    pub fn read_is_motor_mode(&self) -> bool {
        // This is a simplification since we don't have direct access to readIsMotorMode()
        // We're returning false as a placeholder - in a real implementation you would
        // need to implement this functionality if needed
        false
    }

    // The vin() function doesn't appear to be in the bindings, so we're using a placeholder
    pub fn vin(&self) -> f32 {
        // In a real implementation, you would need to call the actual voltage reading function
        // For now, we return a placeholder value
        5.0
    }

    pub fn id_read(&self) -> u8 {
        // This is a placeholder as well
        // In a real implementation, you would need to implement this
        1
    }
}

impl Drop for Servo {
    fn drop(&mut self) {
        unsafe { lx16a_servo_destroy(self.ptr) };
    }
}
