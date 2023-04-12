//#define  _GNU_SOURCE

#include <iostream>
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
        co_await std::suspend_always{};

//        std::cout << "async put value " << context->putValue << std::endl;
        context->atomics->serverPtr->store( context->putValue, std::memory_order_relaxed );

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
        context.putValue = context.atomics->spinUntilClientChange( context.putValue );
//        std::cout << "main value = " << context.putValue << std::endl;
        handle();
    }

    return 0;
}




