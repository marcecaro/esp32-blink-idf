
cmake_minimum_required(VERSION 3.5)

# dependencies are component from the one installed in idf_component.yml -> installed in .embuild/
set(PROJECT_DEPS    pthread    
                    driver 
                    vfs
                    arduino-esp32
                    )

# Add our implementation files to the sources
idf_component_register(
    SRCS "src/lx16a-servo.cpp" "src/lx16a_c_wrapper.cpp"
    INCLUDE_DIRS "src"
    PRIV_REQUIRES "${PROJECT_DEPS}"
    WHOLE_ARCHIVE TRUE
    )
