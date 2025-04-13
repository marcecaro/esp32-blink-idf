// Plain C implementation for ESP32
#include "main.h"

static const char* TAG = "ESP32_BLINK";

// Standard C entry point with full ESP-IDF initialization
void app_main(void) {
    ESP_LOGI(TAG, "Initializing ESP-IDF in app_main");
    
    // Initialize NVS - essential for ESP-IDF
    esp_err_t ret = nvs_flash_init();
    if (ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        ret = nvs_flash_init();
        ESP_ERROR_CHECK(ret);
    }
    
    ESP_LOGI(TAG, "ESP-IDF initialization complete");
    
    // Hand over control to Rust - rust_main should contain the main application logic
    ESP_LOGI(TAG, "Transferring control to Rust");
    rust_main();
    
    // If rust_main returns, we can end app_main or do other cleanup
    ESP_LOGI(TAG, "Control returned from Rust to C");
}

// Simple C function for use with Rust FFI
int hello(int value) {
    ESP_LOGI(TAG, "Hello from C: %d", value);
    return value*10;
}