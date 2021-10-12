//#define  _GNU_SOURCE

#include <iostream>
#include <sched.h>
#include <cstring>
#include <c++/9/functional>
#include <optional>
#include "MappedAtomics.h"

class Worker {
private:
    std::atomic<long unsigned int> *writePtr;
    long unsigned int someState;
public:
    Worker(std::atomic<long unsigned int> *writePtr) : writePtr(writePtr), someState(0)
    {}

    void doWork( long unsigned int new_value ) {
        writePtr->store( new_value, std::memory_order_relaxed );
        someState = new_value;
    }
};

class EventLoop {
private:
    MappedAtomics *atomics;
    std::optional< std::function<void(long unsigned int)> > callback;
public:
    EventLoop(MappedAtomics *atomics) : atomics(atomics), callback(std::nullopt)
    {}

    void setCallback(std::function<void(long unsigned int)> callback) {
        this->callback.emplace(callback);
    }

    void run() {
        long unsigned int value = 0;
        while(true) {
            value = atomics->spinUntilClientChange(value);
            if( callback ) (*callback)(value);
        }
    }
};

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

    EventLoop ev(atomic);

    Worker wk(atomic->serverPtr);

    ev.setCallback([&wk](long unsigned int value) -> void {
       wk.doWork(value);
    });

    ev.run();

    return 0;
}




