//#define  _GNU_SOURCE

#include <iostream>
#include <cstring>
#include "MappedAtomics.h"

#define UU __attribute__((unused))
int main(UU int argc, UU char * argv[]) {

    auto *atomic = new(MappedAtomics);

    long unsigned int value = 0;

    while(true) {
        value = atomic->spinUntilClientChange( value );
        atomic->serverPtr->store( value, std::memory_order_relaxed );
    }

    return 0;
}




