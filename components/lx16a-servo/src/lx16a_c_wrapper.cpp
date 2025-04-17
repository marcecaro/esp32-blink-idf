#include "lx16a-servo.h"
#include "lx16a_c_wrapper.h"

struct LX16ABusHandle   { LX16ABus   impl; };
struct LX16AServoHandle { LX16AServo impl; };

/* ---------- bus ---------- */
LX16ABusHandle *lx16a_bus_create(void    *serial,
                                 uint32_t baud,
                                 int      tx_pin,
                                 int      tx_flag_gpio)
{
    if (!serial) return nullptr;
    auto *h = new LX16ABusHandle;
    h->impl.begin(static_cast<HardwareSerial *>(serial), tx_pin, tx_flag_gpio);
    h->impl.setRetryCount(3);
    return h;
}
void lx16a_bus_destroy(LX16ABusHandle *h)               { delete h; }

void lx16a_bus_debug      (LX16ABusHandle *h, bool on)  { h->impl.debug(on); }
void lx16a_bus_set_retries(LX16ABusHandle *h, uint8_t n){ h->impl.setRetryCount(n); }
void lx16a_bus_disable_all(LX16ABusHandle *h)           { h->impl.disableAll(); }
uint32_t lx16a_bus_time_ms(LX16ABusHandle *h, uint32_t n){ return h->impl.time(n); }
uint32_t lx16a_bus_time_us(LX16ABusHandle *h, uint32_t n){ return h->impl.timeus(n); }
bool lx16a_bus_write(LX16ABusHandle *h,const uint8_t *b,uint32_t l){return h->impl.write(b,l);}
int  lx16a_bus_read (LX16ABusHandle *h,uint8_t *b,uint32_t l)      {return h->impl.read (b,l);}

/* ---------- servo ---------- */
LX16AServoHandle *lx16a_servo_create(LX16ABusHandle *bus, uint8_t id)
{
    if (!bus) return nullptr;
    auto *s = new LX16AServoHandle{ LX16AServo(&bus->impl, id) };
    s->impl.initialize();              // optional – safe‑guard
    return s;
}
void lx16a_servo_destroy(LX16AServoHandle *s)           { delete s; }

bool    lx16a_servo_move_time (LX16AServoHandle *s, int32_t cd, uint16_t ms){return s->impl.move_time(cd,ms);}
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

void lx16a_servo_set_id    (LX16AServoHandle *s, uint8_t id)  { s->impl.set_id(id); }
void lx16a_servo_motor_mode(LX16AServoHandle *s, bool en)     { s->impl.setMotorMode(en); }
void lx16a_servo_load      (LX16AServoHandle *s, bool en)     { s->impl.load_or_unload(en); }

bool    lx16a_servo_cmd_ok     (LX16AServoHandle *s)          { return s->impl.isCommandOk(); }
int32_t lx16a_servo_temperature(LX16AServoHandle *s)          { return s->impl.temp_read(); }
