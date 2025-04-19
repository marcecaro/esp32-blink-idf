#!/bin/bash
# Enhanced debug script for ESP32 with UART GDBStub

# Ensure the script is executable
if [ ! -x "$0" ]; then
    chmod +x "$0"
fi

# Make sure device is connected
if [ ! -c "/dev/ttyUSB0" ]; then
    echo "Error: Device not found at /dev/ttyUSB0"
    echo "Please connect your ESP32 and try again"
    exit 1
fi

# Reset the device first and properly trigger GDBStub
echo "Preparing ESP32 for debugging..."

# First reset the device (optional if you're reflashing anyway)
stty -F /dev/ttyUSB0 115200
echo -e "\x03" > /dev/ttyUSB0  # Send Ctrl+C to trigger break
sleep 1

# Path to GDB
GDB_PATH=".embuild/espressif/esp-idf/v5.4.1/tools/tools/xtensa-esp-elf-gdb/14.2_20240403/xtensa-esp-elf-gdb/bin/xtensa-esp32-elf-gdb"
ELF_PATH="target/xtensa-esp32-espidf/debug/esp32-blink-idf"

# Start GDB with improved settings
$GDB_PATH $ELF_PATH \
    -ex "set remotetimeout 250" \
    -ex "set serial baud 115200" \
    -ex "set confirm off" \
    -ex "monitor reset halt" \
    -ex "set mem inaccessible-by-default off" \
    -ex "target remote /dev/ttyUSB0" \
    -ex "interrupt" \
    -ex "set pagination off" \
    -ex "b app_main" 
