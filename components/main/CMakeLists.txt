# Use arduino component directly
idf_component_register(SRCS "main.c"
                  INCLUDE_DIRS "."
                  REQUIRES arduino-esp32)
