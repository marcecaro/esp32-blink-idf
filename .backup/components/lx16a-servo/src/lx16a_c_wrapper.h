#pragma once
/*
 * Pure‑C API for LewanSoul LX‑16A half‑duplex servos.
 * Wraps the C++ classes in lx16a‑servo.h so that they look like C structs.
 *
 * Compile lx16a_c_wrapper.cpp as C++17 and link it together with
 * the original lx16a‑servo.cpp.  Down‑stream users only see this file.
 */
#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

/* ---------- opaque handles ---------- */
typedef struct LX16ABusHandle   LX16ABusHandle;
typedef struct LX16AServoHandle LX16AServoHandle;
typedef struct HardwareSerial HardwareSerial;

HardwareSerial *getSerial();
HardwareSerial *getSerial1();
HardwareSerial *getSerial2();

void HardwareSerial_begin(HardwareSerial *, uint32_t baud);
void HardwareSerial_end(HardwareSerial *);

/* ---------- bus life‑cycle ---------- */
LX16ABusHandle *lx16a_bus_create();
void            lx16a_bus_destroy(LX16ABusHandle *bus);


/* ---------- bus helpers ---------- */

void lx16a_bus_beginOnePinMode(LX16ABusHandle *bus, HardwareSerial * port, int tXrXpin);

void     lx16a_bus_debug      (LX16ABusHandle *bus, bool on);
void     lx16a_bus_set_retries(LX16ABusHandle *bus, uint8_t n);
void     lx16a_bus_disable_all(LX16ABusHandle *bus);
uint32_t lx16a_bus_time_ms    (LX16ABusHandle *bus, uint32_t n_bytes);
uint32_t lx16a_bus_time_us    (LX16ABusHandle *bus, uint32_t n_bytes);
bool lx16a_bus_write(LX16ABusHandle *bus,
    uint8_t         cmd,
    const uint8_t  *params,
    int             param_cnt,
    uint8_t         id);
bool lx16a_bus_read (LX16ABusHandle *bus,
                     uint8_t         cmd,
                     uint8_t        *params,
                     int             param_len,
                     uint8_t         id);

/* ---------- servo life‑cycle ---------- */
LX16AServoHandle *lx16a_servo_create (LX16ABusHandle *bus, uint8_t id);
void              lx16a_servo_destroy(LX16AServoHandle *s);

/* ---------- servo motion / control ---------- */
void lx16a_servo_initialize(LX16AServoHandle *s);
void     lx16a_servo_move_time (LX16AServoHandle *s, int32_t cent_deg, uint16_t time_ms);
int32_t  lx16a_servo_pos_read  (LX16AServoHandle *s);
int32_t  lx16a_servo_pos_cached(LX16AServoHandle *s);

void     lx16a_servo_set_limits(LX16AServoHandle *s,
                                int32_t min_cent_deg,
                                int32_t max_cent_deg);
bool     lx16a_servo_calibrate (LX16AServoHandle *s,
                                int32_t current_cent_deg,
                                int32_t min_cent_deg,
                                int32_t max_cent_deg);

void     lx16a_servo_set_id    (LX16AServoHandle *s, uint8_t new_id);        // uses id_write
void     lx16a_servo_motor_mode(LX16AServoHandle *s, int16_t speed);         // speed  (-1000..+1000)
void     lx16a_servo_load      (LX16AServoHandle *s, bool enable);

bool     lx16a_servo_cmd_ok    (LX16AServoHandle *s);  
int32_t  lx16a_servo_temperature(LX16AServoHandle *s);


float lx16a_servo_vin(LX16AServoHandle *s);
float lx16a_servo_id_read(LX16AServoHandle *s);
bool lx16a_servo_read_is_motor_mode(LX16AServoHandle *s);
#ifdef __cplusplus
} /* extern "C" */
#endif
