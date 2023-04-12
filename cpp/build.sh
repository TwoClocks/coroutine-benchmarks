CMD="zig c++ -std=c++20 -lrt -Wall -Wextra -Wconversion -Wsign-conversion -Ofast -finline-functions"

`mkdir -p out`

`$CMD -oout/atomicSpin atomicSpin.cpp MappedAtomics.cpp`
`$CMD -oout/asyncResume asyncResume.cpp MappedAtomics.cpp`
`$CMD -oout/asyncSuspend asyncSuspend.cpp MappedAtomics.cpp`
`$CMD -oout/atomicCallback atomicCallback.cpp MappedAtomics.cpp`
