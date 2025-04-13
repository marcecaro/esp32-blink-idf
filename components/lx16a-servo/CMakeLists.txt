
cmake_minimum_required(VERSION 3.5)


# dependencies are component from the one installed in idf_component.yml -> installed in .embuild/
set(PROJECT_DEPS    pthread    
                    driver 
                    vfs
                    )



idf_component_register(
    SRCS "src/lx16a-servo.cpp"
    INCLUDE_DIRS "src"
    PRIV_REQUIRES "${PROJECT_DEPS}"
    #WHOLE_ARCHIVE TRUE
)
