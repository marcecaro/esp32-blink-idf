#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("../components/lx16a-servo/src/lx16a-servo.h");
    
        type LX16ABus;
        type LX16AServo;

        //  Functions implemented in C++.
        fn new_bus(tx_pin: u8, rx_pin: u8) -> UniquePtr<LX16ABus>;
        fn new_servo(bus: &LX16ABus, id: u8) -> UniquePtr<LX16AServo>;
        
        // Existing methods
        fn move_servo(self: &LX16AServo, position: i16, time: u16);
        fn get_position(self: &LX16AServo) -> i16;
        fn get_id(self: &LX16AServo) -> u8;
        fn get_bus(self: &LX16AServo) -> &LX16ABus;
        fn calibrate(self: &LX16AServo, current_angle_cent_deg: i32, min_angle_cent_deg: i32, max_angle_cent_deg: i32) -> bool;
        fn initialize(self: &LX16AServo);
        fn pos_read(self: &LX16AServo) -> i32;
        fn pos_read_cached(self: &LX16AServo) -> i32;
        fn read_angle_offset(self: &LX16AServo) -> i16;
        fn angle_offset_adjust(self: &LX16AServo, angle: i16);
        fn angle_offset_save(self: &LX16AServo);
        fn motor_mode(self: &LX16AServo, speed: i16);
        fn setLimitsTicks(self: &LX16AServo, lower: i32, upper: i32);
        fn getMinCentDegrees(self: &LX16AServo) -> i32;
        fn getMaxCentDegrees(self: &LX16AServo) -> i32;
        fn isCommandOk(self: &LX16AServo) -> bool;
        fn readLimits(self: &LX16AServo);
        fn move_time(self: &LX16AServo, angle: i32, time: u16);
        fn move_time_and_wait_for_sync(self: &LX16AServo, angle: i32, time: u16);
        fn stop(self: &LX16AServo);
        fn disable(self: &LX16AServo);
        fn enable(self: &LX16AServo);
        fn id_write(self: &LX16AServo, id: u8);
        fn id_verify(self: &LX16AServo) -> u8;
        fn readIsMotorMode(self: &LX16AServo) -> bool;
        fn temp(self: &LX16AServo) -> u8;
        fn vin(self: &LX16AServo) -> u16;
    }
}
