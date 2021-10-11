//#define  _GNU_SOURCE

#include <iostream>
#include <sched.h>
#include <cstring>
#include "MappedAtomics.h"
#include <concepts>
#include <coroutine>


struct AsyncContext {
    MappedAtomics * atomics;
    long unsigned int putValue;
};

struct CoroutineHandle {
    struct promise_type {
        CoroutineHandle get_return_object() {
            return {
                    // Uses C++20 designated initializer syntax
                    .h_ = std::coroutine_handle<promise_type>::from_promise(*this)
            };
        }
        std::suspend_never initial_suspend() { return {}; }
        std::suspend_never final_suspend() noexcept { return {}; }
        void unhandled_exception() {}
    };

    std::coroutine_handle<promise_type> h_;
    operator std::coroutine_handle<promise_type>() const { return h_; }
    // A coroutine_handle<promise_type> converts to coroutine_handle<>
    operator std::coroutine_handle<>() const { return h_; }
};

CoroutineHandle
asyncLoop( AsyncContext *context )
{

    while( true ) {
        context->putValue = context->atomics->spinUntilClientChange( context->putValue );
//        std::cout << "async value = " << context->putValue << std::endl;
        co_await std::suspend_always{};
    }
}


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


    auto context = AsyncContext{
        .atomics = atomic,
        .putValue = 0
    };

    std::coroutine_handle<> handle = asyncLoop(&context);

    while( true ) {
//        std::cout << "main put value " << context.putValue << std::endl;
        atomic->serverPtr->store( context.putValue, std::memory_order_relaxed );
        handle();
    }

    return 0;
}




