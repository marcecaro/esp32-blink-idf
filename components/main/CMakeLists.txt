cmake_minimum_required(VERSION 3.5)


# dependencies are component from the one installed in idf_component.yml -> installed in .embuild/
set(PROJECT_DEPS    pthread    
                    driver 
                    vfs
                    nvs_flash
                    )


idf_component_register(
    SRCS "main.c"
    INCLUDE_DIRS "."
    PRIV_REQUIRES "${PROJECT_DEPS}"
    WHOLE_ARCHIVE TRUE
)

# Get the directory of the Rust library
set(RUST_LIB_DIR "${CMAKE_SOURCE_DIR}/target/xtensa-esp32-espidf/debug")

# Link with Rust library
target_link_libraries(${COMPONENT_LIB} "${RUST_LIB_DIR}/libesp32blinkirust.a")

# Add linker flags to resolve symbol conflicts
target_link_options(${COMPONENT_LIB} PRIVATE
    "-Wl,--allow-multiple-definition"
    "-Wl,--exclude-libs=libnosys.a"
)

# Add dependency to ensure Rust library is built first
add_custom_target(rust_lib
    COMMAND cargo build --target xtensa-esp32-espidf
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    COMMENT "Building Rust library"
)

add_dependencies(${COMPONENT_LIB} rust_lib)
