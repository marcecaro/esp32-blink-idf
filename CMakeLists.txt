cmake_minimum_required(VERSION 3.5)
# Add at the top of your CMakeLists.txt
#set(CMAKE_VERBOSE_MAKEFILE ON CACHE BOOL "Verbose build output" FORCE)
# Add to your main CMakeLists.txt
#set(VERBOSE 1)

# Add project components directory to the search path
set(COMPONENT_DIRS ${CMAKE_CURRENT_LIST_DIR}/components/ $ENV{IDF_PATH}/components)

# Include the ESP-IDF project.cmake file
include($ENV{IDF_PATH}/tools/cmake/project.cmake)


# Define the path to the Arduino packages
set(ARDUINO_PACKAGES_PATH "/home/mcaro/workbench/arduino-packages/libraries")

# Recursively find all .cpp and .c files in the specified path
file(GLOB_RECURSE LIBRARY_SRCS
    "${ARDUINO_PACKAGES_PATH}/*.cpp"
    "${ARDUINO_PACKAGES_PATH}/*.c"
)

# set(includedirs
#   variants/${CONFIG_ARDUINO_VARIANT}/
#   cores/esp32/
#   /home/mcaro/workbench/arduino-packages/libraries/SimplePacketComs/src/
# )
file(GLOB_RECURSE LIBRARY_SRCS
    "${ARDUINO_PACKAGES_PATH}/*.cpp"
    "${ARDUINO_PACKAGES_PATH}/*.c"
)

# Recursively find all .h files in the specified path
file(GLOB_RECURSE HEADER_FILES "${ARDUINO_PACKAGES_PATH}/*.h")

# Extract the directories containing the header files
set(INCLUDE_DIRS "")
foreach(header_file ${HEADER_FILES})
    get_filename_component(dir ${header_file} DIRECTORY)
    list(APPEND INCLUDE_DIRS ${dir})
endforeach()

# Remove duplicate directories
list(REMOVE_DUPLICATES INCLUDE_DIRS)

# Add include directories
include_directories(${INCLUDE_DIRS})

# Project declaration
project(esp32-blink-idf)
