cmake_minimum_required(VERSION 3.5)

# Dependencies are components from the one installed in idf_component.yml -> installed in .embuild/
set(PROJECT_DEPS    pthread    
                    driver 
                    vfs
                    nvs_flash
                    arduino-esp32
                    )

idf_component_register(
    SRCS "main.c"
    INCLUDE_DIRS "."
    PRIV_REQUIRES "${PROJECT_DEPS}"
    WHOLE_ARCHIVE TRUE
)
