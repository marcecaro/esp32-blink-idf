Based on: [ESP
-> DF-TEMPLASTE](https://github.com/esp-rs/esp-idf-template/blob/master/README-cmake.md)

This project is an hybrid rust/c++ code base for esp32. The c/c++ is managed IDF-ESP CMakefile and the rust is cargo based.
in build.rs it uses bindgen to create the ffi to call c/c++ from rust code.
build.rs is taking all the includes and definition from "build/compile_commands.json" and using it to pass it to bindgen. 

For some reason, it seem not all definitions or include directories are extracted because i get this error:

$ make cargo-build
 thread 'main' panicked at build.rs:225:10:
  Unable to generate bindings for lx16a-servo: ClangDiagnostic("/home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/newlib/platform_include/endian.h:51:10: fatal error: 'machine/endian.h' file not found\n"

I want to fix it without creating any additional wrapper or without harcodding any include directory.







  "command": "/home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/tools/tools/xtensa-esp-elf/esp-13.2.0_20240530/xtensa-esp-elf/bin/xtensa-esp32-elf-g++ -DESP_MDNS_VERSION_NUMBER=\\\"1.8.2\\\" -DESP_PLATFORM -DIDF_VER=\\\"v5.3.3\\\" -DMBEDTLS_CONFIG_FILE=\\\"mbedtls/esp_config.h\\\" -DSOC_MMU_PAGE_SIZE=CONFIG_MMU_PAGE_SIZE -DSOC_XTAL_FREQ_MHZ=CONFIG_XTAL_FREQ -D_GLIBCXX_HAVE_POSIX_SEMAPHORE -D_GLIBCXX_USE_POSIX_SEMAPHORE -D_GNU_SOURCE -D_POSIX_READER_WRITER_LOCKS 
-> /home/mcaro/workbench/esp32-blink-idf/build/config 
-> /home/mcaro/workbench/esp32-blink-idf/components/lx16a-servo/src 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/newlib/platform_include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/config/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/config/include/freertos 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/config/xtensa/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/FreeRTOS-Kernel/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/FreeRTOS-Kernel/portable/xtensa/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/FreeRTOS-Kernel/portable/xtensa/include/freertos 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/freertos/esp_additions/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/include/soc 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/include/soc/esp32 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/dma/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/ldo/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/port/esp32/. 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_hw_support/port/esp32/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/heap/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/log/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/soc/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/soc/esp32 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/soc/esp32/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/hal/platform_port/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/hal/esp32/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/hal/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_rom/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_rom/include/esp32 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_rom/esp32 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_common/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_system/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_system/port/soc 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_system/port/include/private 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/xtensa/esp32/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/xtensa/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/xtensa/deprecated_include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/include/apps 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/include/apps/sntp 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/lwip/src/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/port/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/port/freertos/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/port/esp32xx/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/port/esp32xx/include/arch 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/lwip/port/esp32xx/include/sys 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/pthread/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/driver/deprecated 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/driver/i2c/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/driver/touch_sensor/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/driver/twai/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/driver/touch_sensor/esp32/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_pm/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_ringbuf/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_gpio/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_pcnt/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_gptimer/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_spi/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_mcpwm/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_ana_cmpr/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_i2s/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_sdmmc/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/sdmmc/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_sdspi/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_sdio/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_dac/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_rmt/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_tsens/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_sdm/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_i2c/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_uart/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/vfs/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_ledc/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_parlio/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_driver_usb_serial_jtag/include 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/variants/esp32 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/cores/esp32 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/ArduinoOTA/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/AsyncUDP/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/BLE/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/BluetoothSerial/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/DNSServer/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/EEPROM/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/ESP_I2S/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/ESP_NOW/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/ESP_SR/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/ESPmDNS/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Ethernet/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/FFat/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/FS/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/HTTPClient/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/HTTPUpdate/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Insights/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/LittleFS/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Matter/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/NetBIOS/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Network/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/OpenThread/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/PPP/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Preferences/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/RainMaker/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/SD_MMC/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/SD/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/SimpleBLE/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/SPIFFS/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/SPI/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Ticker/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Update/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/USB/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/WebServer/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/NetworkClientSecure/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/WiFi/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/WiFiProv/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Wire/src 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__arduino-esp32/libraries/Zigbee/src 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/spi_flash/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_partition/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/port/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/mbedtls/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/mbedtls/library 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/esp_crt_bundle/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/mbedtls/3rdparty/everest/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/mbedtls/3rdparty/p256-m 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/mbedtls/mbedtls/3rdparty/p256-m/p256-m 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/wpa_supplicant/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/wpa_supplicant/port/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/wpa_supplicant/esp_supplicant/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_adc/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_adc/interface 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_adc/esp32/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_adc/deprecated/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_eth/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_event/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/http_parser 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/chmorgan__esp-libhelix-mp3/libhelix-mp3/pub 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__esp-modbus/freemodbus/common/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_timer/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/ieee802154/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_coex/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_netif/include 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__libsodium/libsodium/src/libsodium/include 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__libsodium/port_include 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__mdns/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/console 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_vfs_console/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_https_ota/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_http_client/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/bootloader_support/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/bootloader_support/bootloader_flash/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_app_format/include 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/joltwallet__littlefs/include 
-> /home/mcaro/workbench/esp32-blink-idf/managed_components/espressif__network_provisioning/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/protocomm/include/common 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/protocomm/include/security 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/protocomm/include/transports 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/protocomm/include/crypto/srp6a 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/protocomm/proto-c 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_wifi/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_wifi/include/local 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_wifi/wifi_apps/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_wifi/wifi_apps/nan_app/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_phy/include 
-> /home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3/components/esp_phy/esp32/include -mlongcalls -Wno-frame-address  -fno-builtin-memcpy -fno-builtin-memset -fno-builtin-bzero -fno-builtin-stpcpy -fno-builtin-strncpy -fdiagnostics-color=always -ffunction-sections -fdata-sections -Wall -Werror=all -Wno-error=unused-function -Wno-error=unused-variable -Wno-error=unused-but-set-variable -Wno-error=deprecated-declarations -Wextra -Wno-unused-parameter -Wno-sign-compare -Wno-enum-conversion -gdwarf-4 -ggdb -Og -fno-shrink-wrap -fmacro-prefix-map=/home/mcaro/workbench/esp32-blink-idf=. -fmacro-prefix-map=/home/mcaro/workbench/esp32-blink-idf/.embuild/espressif/esp-idf/v5.3.3=/IDF -fstrict-volatile-bitfields -fno-jump-tables -fno-tree-switch-conversion -std=gnu++2b -fno-exceptions -fno-rtti -DARDUINO=10812 -DARDUINO_ESP32_DEV -DARDUINO_ARCH_ESP32 -DARDUINO_BOARD=\\\"ESP32_DEV\\\" -DARDUINO_VARIANT=\\\"esp32\\\" -DESP32=ESP32 -o esp-idf/lx16a-servo/CMakeFiles/__idf_lx16a-servo.dir/src/lx16a-servo.cpp.obj -c /home/mcaro/workbench/esp32-blink-idf/components/lx16a-servo/src/lx16a-servo.cpp",
  "file": "/home/mcaro/workbench/esp32-blink-idf/components/lx16a-servo/src/lx16a-servo.cpp",





# Layout

The project is structured as follows:

```
.
├── components
│   ├── esp32servoserver  # cpp dependencies
│   |     ├── idf_component.yml # IDF component manifest, downloads them, needs to apear in CMakeLists
│   |     ├── CMakeLists.txt    # CMake configuration for CPP dependencies
│   |     └── include
│   |  
│   └── lx16a-servo
├── Cargo.toml            # Rust dependencies   
├── Makefile              # High level Makefile
├── README.md
├── SETUP-ENVIRONMENT.md  # Setup environment
├── target
├── wokwi.toml              # Wokwi configuration, emulator
├── src                     # Rust source code
├── CMakeLists.txt          # CMake configuration for CPP dependencies
└── project-export.sh       # Environment variables
```


# Setup environment

sudo apt install clang

Install:
    - ROS JAZZY
    - Rust

First run the setup script, to install the local ESP
-> DF, to build the components directory:

```bash
make bootstrap
```
Then open a new terminal.

**bash.rc:**
```shell

## ROS2 Jazzy
source /opt/ros/jazzy/setup.bash && echo "ROS2 Jazzy Environment loaded successfully"
export PATH=$HOME/.local/bin:$PATH

#source $HOME/esp/esp-idf/export.sh 
source $HOME/.local/bin/env echo "~/local/bin/env  Environment loaded successfully"
source $HOME/.cargo/env && echo "Cargo Environment loaded successfully"


# Auto-source project-export.sh if it exists in the current directory
function check_project_export() {
  if [ -f "./project-export.sh" ]; then
    echo "Found project-export.sh in current directory, sourcing it..."
    source ./project-export.sh
  fi
}

check_project_export;
```


install the dependencies:

```bash
cargo update
make refresh-idf-deps
```

### Add idf-dependencies

In the component that needs the dependency, check the idf_component.yml file
Run `make` to download the dependency

### Add cpp-dependencies

- Check components/esp32servoserver

### Add cpp-dependencies from arduino registry

- Point the Arduino IDE to the ~/workbench/arduino-packages directory
- Insatall the package there
- is needed to add the idf dependency: espressif/arduino-esp32


### Generates Binding From .h to rust

```bash
mkdir -p src/lx16a
bindgen components/lx16a-servo/src/lx16a-servo.h -o src/lx16a/mod.rs
```

# Build

```bash
make build-idf
make build-cargo
```

# Flash

```bash
cargo flash
```

# Monitor

```bash
cargo monitor
```