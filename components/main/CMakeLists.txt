
cmake_minimum_required(VERSION 3.5)


# dependencies are component from the one installed in idf_component.yml -> installed in .embuild/
set(PROJECT_DEPS    pthread    
                    driver 
                    vfs
                    )



idf_component_register(
    SRCS "main.cpp"
    INCLUDE_DIRS "."
    PRIV_REQUIRES "${PROJECT_DEPS}"
    WHOLE_ARCHIVE TRUE
)
