
// Re-export the FFI bindings
#[allow(warnings)]
pub mod ffi;
pub use ffi::*;

pub struct LX16AHardwareSerial {
    ptr: *mut HardwareSerial,
}

impl LX16AHardwareSerial {
    #[allow(dead_code)]
    pub fn begin(&mut self, baud: u32) -> () {
        unsafe { HardwareSerial_begin(self.ptr, baud) };
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { ptr: unsafe { getSerial() } }
    }
    #[allow(dead_code)]
    pub fn new_1() -> Self {
        Self { ptr: unsafe { getSerial1() } }
    }
    #[allow(dead_code)]
    pub fn new_2() -> Self {
        Self { ptr: unsafe { getSerial2() } }
    }

}

impl Drop for LX16AHardwareSerial {
    fn drop(&mut self) {
        unsafe { HardwareSerial_end(self.ptr) };
    }
}

// A safe wrapper around the LX16ABus pointer
pub struct ServoBus {
    ptr: *mut LX16ABusHandle,
}


impl ServoBus {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let ptr = unsafe { lx16a_bus_create() };
        Self { ptr }
    }

    #[allow(dead_code)]
    pub fn debug(&self, enable: bool) {
        unsafe { lx16a_bus_debug(self.ptr, enable) };
    }

    #[allow(dead_code)]
    pub fn begin_one_pin_mode(&self, serial: &LX16AHardwareSerial, tx_pin: i32) {
        unsafe { lx16a_bus_beginOnePinMode(self.ptr, serial.ptr, tx_pin) };
    }

    #[allow(dead_code)]
    pub fn set_retries(&self, n: u8) {
        unsafe { lx16a_bus_set_retries(self.ptr, n) };
    }

    // pub fn disable_all(&self) {
    //     unsafe { lx16a_bus_disable_all(self.ptr) };
    // }

    // pub fn time_ms(&self, n_bytes: u32) -> u32 {
    //     unsafe { lx16a_bus_time_ms(self.ptr, n_bytes) }
    // }

    // pub fn time_us(&self, n_bytes: u32) -> u32 {
    //     unsafe { lx16a_bus_time_us(self.ptr, n_bytes) }
    // }

    // pub fn write(&self, cmd: u8, params: *const u8, param_cnt: i32, id: u8) -> bool {
    //     unsafe { lx16a_bus_write(self.ptr, cmd, params, param_cnt, id) }
    // }

    // pub fn read(&self, cmd: u8, params: *mut u8, param_len: i32, id: u8) -> bool {
    //     unsafe { lx16a_bus_read(self.ptr, cmd, params, param_len, id) }
    // }

    // pub fn read_no_retry(&self, cmd: u8, params: *mut u8, param_len: i32, id: u8) -> bool {
    //     unsafe { lx16a_bus_read_no_retry(self.ptr, cmd, params, param_len, id) }
    // }
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
        unsafe { lx16a_servo_read_is_motor_mode(self.ptr) }
    }

    // The vin() function doesn't appear to be in the bindings, so we're using a placeholder
    pub fn vin(&self) -> f32 {
        // In a real implementation, you would need to call the actual voltage reading function
        // For now, we return a placeholder value
        unsafe { lx16a_servo_vin(self.ptr) }
    }

    pub fn id_read(&self) -> f32 {
        // This is a placeholder as well
        // In a real implementation, you would need to implement this
        unsafe { lx16a_servo_id_read(self.ptr) }
    }

    pub fn initialize(&self) {
        unsafe { lx16a_servo_initialize(self.ptr) };
    }
}

impl Drop for Servo {
    fn drop(&mut self) {
        unsafe { lx16a_servo_destroy(self.ptr) };
    }
}
