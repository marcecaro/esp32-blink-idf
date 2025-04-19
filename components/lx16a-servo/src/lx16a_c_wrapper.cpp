#include "lx16a-servo.h"
#include "lx16a_c_wrapper.h"
#include <HardwareSerial.h>

struct LX16ABusHandle   { LX16ABus   impl; };
struct LX16AServoHandle { LX16AServo impl; };
 

HardwareSerial *getSerial(){
    return &Serial;
};

HardwareSerial *getSerial1() {
    return &Serial1;
}

HardwareSerial *getSerial2() {
    return &Serial2;
};


void HardwareSerial_begin(HardwareSerial *impl, uint32_t baud) {

    impl->begin(baud);
};

void HardwareSerial_end(HardwareSerial *impl) {
    impl->end();
}

/* ---------- bus ---------- */
LX16ABusHandle *lx16a_bus_create()
{
    return new LX16ABusHandle{LX16ABus()};
}
void lx16a_bus_destroy(LX16ABusHandle *h)               { delete h; }

void lx16a_bus_debug      (LX16ABusHandle *h, bool on)  { h->impl.debug(on); }
void lx16a_bus_set_retries(LX16ABusHandle *h, uint8_t n){ h->impl.setRetryCount(n); }
void lx16a_bus_disable_all(LX16ABusHandle *h)           { h->impl.disableAll(); }
uint32_t lx16a_bus_time_ms(LX16ABusHandle *h, uint32_t n){ return h->impl.time(n); }
uint32_t lx16a_bus_time_us(LX16ABusHandle *h, uint32_t n){ return h->impl.timeus(n); }


void lx16a_bus_beginOnePinMode(LX16ABusHandle *bus, HardwareSerial * port, int tXrXpin){
    bus->impl.beginOnePinMode(port, tXrXpin);
}


/* ---------- servo ---------- */
LX16AServoHandle *lx16a_servo_create(LX16ABusHandle *bus, uint8_t id)
{
    if (!bus) return nullptr;
    auto *s = new LX16AServoHandle{ LX16AServo(&bus->impl, id) };            // optional – safe‑guard
    return s;
}
void lx16a_servo_destroy(LX16AServoHandle *s)           { delete s; }
void lx16a_servo_initialize(LX16AServoHandle *s) { s->impl.initialize(); }
int32_t lx16a_servo_pos_read  (LX16AServoHandle *s)                        {return s->impl.pos_read();}
int32_t lx16a_servo_pos_cached(LX16AServoHandle *s)                        {return s->impl.pos_read_cached();}

void lx16a_servo_set_limits(LX16AServoHandle *s,int32_t lo,int32_t hi)
{
    s->impl.setLimitsTicks((lo-s->impl.staticOffset)/24,
                           (hi-s->impl.staticOffset)/24);
}
bool lx16a_servo_calibrate(LX16AServoHandle *s,
                           int32_t cur,int32_t lo,int32_t hi)
{ return s->impl.calibrate(cur,lo,hi); }


bool    lx16a_servo_cmd_ok     (LX16AServoHandle *s)          { return s->impl.isCommandOk(); }
int32_t lx16a_servo_temperature(LX16AServoHandle *s)          { return s->impl.temp(); }




/* ---------- bus helpers ---------- */
bool lx16a_bus_write(LX16ABusHandle *h,
    uint8_t         cmd,
    const uint8_t  *params,
    int             cnt,
    uint8_t         id)
{
return h->impl.write(cmd, params, cnt, id);
}

bool lx16a_bus_read(LX16ABusHandle *h,
   uint8_t         cmd,
   uint8_t        *params,
   int             len,
   uint8_t         id)
{
return h->impl.read(cmd, params, len, id);
}

/* ---------- servo motion ---------- */
void lx16a_servo_move_time(LX16AServoHandle *s,
          int32_t          cent_deg,
          uint16_t         time_ms)
{
s->impl.move_time(cent_deg, time_ms);
}

/* ---------- misc helpers ---------- */
void lx16a_servo_set_id(LX16AServoHandle *s, uint8_t id)
{
s->impl.id_write(id);
}

void lx16a_servo_motor_mode(LX16AServoHandle *s, int16_t speed)
{
s->impl.motor_mode(speed);
}

float lx16a_servo_vin(LX16AServoHandle *s) {
    return s->impl.vin();
}

float lx16a_servo_id_read(LX16AServoHandle *s) {
    return s->impl.id_read();
}

bool lx16a_servo_read_is_motor_mode(LX16AServoHandle *s) {
    return s->impl.readIsMotorMode();
}