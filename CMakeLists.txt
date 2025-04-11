cmake_minimum_required(VERSION 3.5)
#set(VERBOSE 1)


include($ENV{IDF_PATH}/tools/cmake/project.cmake)

# Add project components directory to the search path
set(COMPONENT_DIRS ${CMAKE_CURRENT_LIST_DIR}/components/ $ENV{IDF_PATH}/components)

# Project declaration
project(esp32-blink-idf)