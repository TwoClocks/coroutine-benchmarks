cmake_minimum_required(VERSION 3.20)
project(cpp)

set(CMAKE_CXX_STANDARD 20)
add_compile_options(
        -Wall
        -Wextra
        -Wconversion
        -Wsign-conversion
        $<$<CONFIG:DEBUG>:-g3>
        $<$<CONFIG:DEBUG>:-Og>
        $<$<CONFIG:RELEASE>:-Ofast>
        $<$<CONFIG:RELEASE>:-finline-functions>
)

add_executable(cpp main.cpp MappedAtomics.cpp MappedAtomics.h)
TARGET_LINK_LIBRARIES(cpp rt)
