#pragma once

#include <stdint.h>
#include <stdbool.h>

// Forward declaration of Arduino types we need
class HardwareSerial;

// Constants from lx16a-servo.h
#define LX16A_BROADCAST_ID 0xFE
#define LX16A_SERVO_MOVE_TIME_WRITE 1
#define LX16A_SERVO_MOVE_TIME_READ 2
#define LX16A_SERVO_MOVE_TIME_WAIT_WRITE 7
#define LX16A_SERVO_MOVE_TIME_WAIT_READ 8
#define LX16A_SERVO_MOVE_START 11
#define LX16A_SERVO_MOVE_STOP 12
#define LX16A_SERVO_ID_WRITE 13
#define LX16A_SERVO_ID_READ 14
#define LX16A_SERVO_ANGLE_OFFSET_ADJUST 17
#define LX16A_SERVO_ANGLE_OFFSET_WRITE 18
#define LX16A_SERVO_ANGLE_OFFSET_READ 19
#define LX16A_SERVO_ANGLE_LIMIT_WRITE 20
#define LX16A_SERVO_ANGLE_LIMIT_READ 21
#define LX16A_SERVO_VIN_LIMIT_WRITE 22
#define LX16A_SERVO_VIN_LIMIT_READ 23
#define LX16A_SERVO_TEMP_MAX_LIMIT_WRITE 24
#define LX16A_SERVO_TEMP_MAX_LIMIT_READ 25
#define LX16A_SERVO_TEMP_READ 26
#define LX16A_SERVO_VIN_READ 27
#define LX16A_SERVO_POS_READ 28
#define LX16A_SERVO_OR_MOTOR_MODE_WRITE 29
#define LX16A_SERVO_OR_MOTOR_MODE_READ 30
#define LX16A_SERVO_LOAD_OR_UNLOAD_WRITE 31
#define LX16A_SERVO_LOAD_OR_UNLOAD_READ 32
#define LX16A_SERVO_LED_CTRL_WRITE 33
#define LX16A_SERVO_LED_CTRL_READ 34
#define LX16A_SERVO_LED_ERROR_WRITE 35
#define LX16A_SERVO_LED_ERROR_READ 36

// Forward declarations with minimal necessary method signatures for FFI
class LX16ABus {
public:
    // Constructor and initialization
    LX16ABus();
    void debug(bool on);
    void begin(HardwareSerial* port, int tXpin, int TXFlagGPIO = -1);
    void beginOnePinMode(HardwareSerial* port, int tXrXpin);
    
    // Basic I/O methods
    bool available();
    int read();
    void write(const uint8_t* buf, int buflen);
    uint32_t time(uint32_t n);
    uint32_t timeus(uint32_t n);
    
    // Command methods
    bool write(uint8_t cmd, const uint8_t* params, int param_cnt, uint8_t MYID);
    bool read(uint8_t cmd, uint8_t* params, int param_len, uint8_t MYID);
    bool write_no_retry(uint8_t cmd, const uint8_t* params, int param_cnt, uint8_t MYID);
    bool read_no_retry(uint8_t cmd, uint8_t* params, int param_len, uint8_t MYID);
    bool rcv(uint8_t cmd, uint8_t* params, int param_len, uint8_t MYID);
    
    // Control methods
    void setRetryCount(int count);
    bool disableAll();
    bool enableAll();
    bool move_sync_start();
    void stopAll();
    uint8_t id_read();
    void id_write(uint8_t id);
};

class LX16AServo {
public:
    // Public fields
    int32_t staticOffset;
    int32_t maxCentDegrees;
    int32_t minCentDegrees;
    uint8_t _id;
    
    // Constructor and initialization
    LX16AServo(LX16ABus* bus, int id);
    void initialize();
    bool isCommandOk();
    
    // Calibration and limits
    bool calibrate(int32_t currentAngleCentDegrees, int32_t min_angle_cent_deg, int32_t max_angle_cent_deg);
    void setLimitsTicks(int32_t lower, int32_t upper);
    void readLimits();
    int32_t getMinCentDegrees();
    int32_t getMaxCentDegrees();
    
    // Movement control
    void move_time(int32_t angle, uint16_t time);
    void move_time_and_wait_for_sync(int32_t angle, uint16_t time);
    void stop();
    void disable();
    void enable();
    
    // Mode control
    void motor_mode(int16_t speed);
    
    // Position and angle methods
    int32_t pos_read();
    int32_t pos_read_cached();
    void angle_offset_adjust(int16_t angle);
    void angle_offset_save();
    int16_t read_angle_offset();

    // ID methods
    uint8_t id_read();
    uint8_t id_verify();
};

// Include the actual header with full implementation
// This ensures C++ code can still use the full implementation
// // while bindgen only sees the forward declarations
// #ifdef __cplusplus
// #include "lx16a-servo.h"
// #endif
