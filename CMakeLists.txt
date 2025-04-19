cmake_minimum_required(VERSION 3.16)
#set(VERBOSE 1)

# # Add the global linker flags before including the project
# set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,--allow-multiple-definition")

include($ENV{IDF_PATH}/tools/cmake/project.cmake)

# Add project components directory to the search path
# set(COMPONENT_DIRS ${CMAKE_CURRENT_LIST_DIR} $ENV{IDF_PATH}/components)

# Project declaration
project(esp32-blink-idf)