//#define  _GNU_SOURCE

#include <iostream>
#include <sched.h>
#include <cstring>
#include "MappedAtomics.h"

int main(int argc, char * argv[]) {

    if(argc < 2) {
        std::cout << "must pass cpu # to run on. exiting." << std::endl;
        return -1;
    }
    auto cpuNum = std::stoul( argv[1]);

    cpu_set_t cpuSet;
    CPU_ZERO( &cpuSet );
    CPU_SET(cpuNum, &cpuSet);
    if( sched_setaffinity(0, sizeof(cpuSet), &cpuSet) != 0 ) {
        std::cout << "set affinity failed with following error : " << std::strerror(errno) << std::endl;
        return -1;
    }

    auto *atomic = new(MappedAtomics);

    long unsigned int value = 0;

    while(true) {
        value = atomic->spinUntilClientChange( value );
        atomic->serverPtr->store( value, std::memory_order_relaxed );
    }

    return 0;
}




