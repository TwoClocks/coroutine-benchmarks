cmake_minimum_required(VERSION 3.20)
project(cpp)

set(CMAKE_CXX_STANDARD 20)
add_compile_options(
        -Wall
        -Wextra
        -Wconversion
        -Wsign-conversion
        -fcoroutines
        -std=c++20
        $<$<CONFIG:DEBUG>:-g3>
        $<$<CONFIG:DEBUG>:-Og>
        $<$<CONFIG:RELEASE>:-Ofast>
        $<$<CONFIG:RELEASE>:-finline-functions>
)

add_executable(atomicSpin atomicSpin.cpp MappedAtomics.cpp MappedAtomics.h)
add_executable(asyncResume asyncResume.cpp MappedAtomics.cpp MappedAtomics.h)
add_executable(asyncSuspend asyncSuspend.cpp MappedAtomics.cpp MappedAtomics.h)
TARGET_LINK_LIBRARIES(atomicSpin rt)
TARGET_LINK_LIBRARIES(asyncResume rt)
TARGET_LINK_LIBRARIES(asyncSuspend rt)
