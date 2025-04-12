#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("../components/lx16a-servo/src/lx16a-servo.h");
    
        type LX16ABus;
        type LX16AServo;
        type HardwareSerial;

        // Constructor functions
        fn new_bus() -> UniquePtr<LX16ABus>;
        fn new_servo(bus: &LX16ABus, id: u8) -> UniquePtr<LX16AServo>;
        
        // LX16ABus methods
        fn debug(self: &LX16ABus, on: bool);
        fn beginOnePinMode(self: &LX16ABus, port: &HardwareSerial, tx_rx_pin: i32);
        fn begin(self: &LX16ABus, port: &HardwareSerial, tx_pin: i32, tx_flag_gpio: i32);
        fn time(self: &LX16ABus, n: u32) -> u32;
        fn timeus(self: &LX16ABus, n: u32) -> u32;
        fn available(self: &LX16ABus) -> bool;
        fn read(self: &LX16ABus) -> i32;
        unsafe fn write(self: &LX16ABus, buf: *const u8, buflen: i32);
        fn setRetryCount(self: &LX16ABus, count: i32);
        unsafe fn write_no_retry(self: &LX16ABus, cmd: u8, params: *const u8, param_cnt: i32, myid: u8) -> bool;
        unsafe fn read_no_retry(self: &LX16ABus, cmd: u8, params: *mut u8, param_len: i32, myid: u8) -> bool;
        unsafe fn rcv(self: &LX16ABus, cmd: u8, params: *mut u8, param_len: i32, myid: u8) -> bool;
        fn disableAll(self: &LX16ABus) -> bool;
        fn enableAll(self: &LX16ABus) -> bool;
        fn move_sync_start(self: &LX16ABus) -> bool;
        fn stopAll(self: &LX16ABus);
        fn id_read(self: &LX16ABus) -> u8;
        fn id_write(self: &LX16ABus, id: u8);
        
        // LX16AServo methods
        fn calibrate(self: &LX16AServo, current_angle_cent_deg: i32, min_angle_cent_deg: i32, max_angle_cent_deg: i32) -> bool;
        fn setLimitsTicks(self: &LX16AServo, lower: i32, upper: i32);
        fn getMinCentDegrees(self: &LX16AServo) -> i32;
        fn getMaxCentDegrees(self: &LX16AServo) -> i32;
        fn isCommandOk(self: &LX16AServo) -> bool;
        fn initialize(self: &LX16AServo);
        fn readLimits(self: &LX16AServo);
        fn move_time(self: &LX16AServo, angle: i32, time: u16);
        fn move_time_and_wait_for_sync(self: &LX16AServo, angle: i32, time: u16);
        fn stop(self: &LX16AServo);
        fn disable(self: &LX16AServo);
        fn enable(self: &LX16AServo);
        fn motor_mode(self: &LX16AServo, speed: i16);
        fn angle_offset_save(self: &LX16AServo);
        fn angle_offset_adjust(self: &LX16AServo, angle: i16);
        fn read_angle_offset(self: &LX16AServo) -> i16;
        fn pos_read(self: &LX16AServo) -> i32;
        fn pos_read_cached(self: &LX16AServo) -> i32;
        fn id_read(self: &LX16AServo) -> u8;
        fn id_verify(self: &LX16AServo) -> u8;
        fn id_write(self: &LX16AServo, id: u8);
        fn readIsMotorMode(self: &LX16AServo) -> bool;
        fn temp(self: &LX16AServo) -> u8;
        fn vin(self: &LX16AServo) -> u16;
        
        // HardwareSerial methods
        fn get_serial() -> &'static HardwareSerial;
        fn begin(self: &HardwareSerial, baud: i32);
        fn begin_with_config(self: &HardwareSerial, baud: i32, config: i32, rx_pin: i32, tx_pin: i32);
    }
}
