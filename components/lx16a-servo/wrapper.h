// Wrapper header for lx16a-servo to avoid ESP-IDF header issues
#pragma once

// Forward declaration of HardwareSerial to avoid including Arduino.h directly
class HardwareSerial;

// Include only the specific component header we need
#include "src/lx16a-servo.h"
