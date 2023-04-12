//#define  _GNU_SOURCE

#include <iostream>
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


#define UU __attribute__((unused))
int main(UU int argc, UU char * argv[]) {

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




