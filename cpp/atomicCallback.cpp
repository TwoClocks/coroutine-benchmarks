//#define  _GNU_SOURCE

#include <iostream>
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

#define UU __attribute__((unused))
int main(UU int argc, UU char * argv[]) {

    auto *atomic = new(MappedAtomics);

    EventLoop ev(atomic);

    Worker wk(atomic->serverPtr);

    ev.setCallback([&wk](long unsigned int value) -> void {
       wk.doWork(value);
    });

    ev.run();

    return 0;
}




