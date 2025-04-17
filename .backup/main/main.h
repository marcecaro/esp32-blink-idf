#ifndef MAIN_H
#define MAIN_H

#include <stdio.h>
#include <stdint.h>
#include "esp_log.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "nvs_flash.h"

// Declare C functions
int hello(int value);

// Declare Rust functions that will be implemented in the Rust library
extern void rust_main(void);

#endif /* MAIN_H */