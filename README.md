# async-benchmarks

At attempt at benchmarking the overhead of async/coroutine IO vs procedural IO in Rust, Zig & Koltin.

#### TLDR; at end
## Background

I used to work at places that specialize in Ultra Low Latency (ULL) development, like Stock Exchanges and High Frequency Trading and the like. As you can imagine the engineering that goes into ULL systems is massive. But you end up having to bypass good abstractions and libraries because of the overhead. A good example is : It is rare ULL code that does a context switch (takes too long). Single threaded only. If you want to communicate w/ another process, you use shared memory. ULL code rarely calls epoll/select either. Best to avoid the kernel as well (evfi,dkdp,openonload,etc). So you end up doing some sort of `while(!hasNewData()) {}` spinning event loop, waiting for memory to change, and when it does you enter callback-hell. The ergonomics suck. Or just give up and learn verilog, put your code on the card w/ the NIC and skip the PCIe bus/kernel/stack all together. Your shinny CPU now just branch predicting a glorified logger.

## Motovation

I recently did another project for a company in an adjacent industry, and I ended up using koltin coroutines for that project (not ULL, but still latency focused). It was great to write procedural looking code that was concurrent. I really enjoyed that project. It made the code so much easier to write / reason about / read. I might have another ULL project coming up, but they don't want to go the FPGA route (or as little as possible). So before that project happens I thought I would measure the overhead of async code vs prodeural. See how much the abstraction costs.
I thought I'd whip up some tests, discover it's too expensive, and go back to my callback-hell cave a little sadder. That is not what happened. That's also why this repo exits. I am unsure of my results and my methodology and would like peer review.

## Methodology

I expected the async overhead to be small, but to exist. So whatever I was doing in a async mannor needed to not dominate the results. So networking code was out. I decided to used shared memory instead of the network and do a ping-pong test. The clint puts a u64 in a memory location one, and times how long it takes that same number to show up in memory location two. The server is spinning waiting for location one to change value, when it does it copies the new value to location two, then loops. Here is the server-side zig code, which I think is the easiest to follow:
```zig
fn runLoop( clientPtr : *const u64, serverPtr : *u64 ) void {
    
    var lastValue : u64 = 0;

    while(true) {
        lastValue = spinUntilChange( clientPtr, lastValue );
        @atomicStore(u64, serverPtr, lastValue, std.builtin.AtomicOrder.Monotonic );
    }
}

fn spinUntilChange( spinPtr:*const u64, lastValue:u64) callconv(.Inline) u64 {

    var newValue = @atomicLoad(u64, spinPtr, std.builtin.AtomicOrder.Monotonic );

    while( newValue == lastValue ) {
        std.atomic.spinLoopHint();
        newValue = @atomicLoad(u64, spinPtr, std.builtin.AtomicOrder.Monotonic );
    }
    return newValue;
}
```

I split the code into two function, because the compilers seemed to like it better that way. I got better numbers with two function. I am not a x86 assembly master, but the resulting ASM from the above looks pretty good to my eyes :
```asm
runLoop:
        xor     eax, eax
        mov     rcx, qword ptr [rdi]
        cmp     rcx, rax
        jne     .LBB0_3
.LBB0_1:
        pause
        mov     rcx, qword ptr [rdi]
        cmp     rcx, rax
        je      .LBB0_1
.LBB0_3:
        mov     qword ptr [rsi], rcx
        mov     rax, rcx
        mov     rcx, qword ptr [rdi]
        cmp     rcx, rax
        je      .LBB0_1
        jmp     .LBB0_3
```

There is only one client, and it's in Rust. Mostly because the criterion benchmarking crate is really great at this kind of microbenchmark stuff. (unless it's not, and that's why my results are so strange).

For the async server code, this is the kotlin version which is probably the easiest to follow. Here is the main spinloop:
```kotlin
val mainLoop = suspend {
    var lastValue = 0L
    while(true) {
        lastValue = loop.getNextValue(lastValue)
        srvBuf.writeLong(0,lastValue)
    }
}
```
`getNextValue()` is a function that just stashes the param, and suspends this coroutine. When the values has changed the coroutine is resumed with the new value. Remember what I said about async coing being easier? It is, unless your writing the event loop. Then it can get a little confusing. But any event loop gets a little confusing. Here is the eventloop:
```kotlin
class SpinEventLoop( val readBuf: Bytes<Void>) {
    private val suspendPoint = CoroutineSuspend<Long>()
    private var lastValue = 0L
    
    suspend fun getNextValue( currentValue:Long ) : Long {
        lastValue = currentValue
        return suspendPoint.suspendMe()
    }
    fun spinLoop() {
        var value = readBuf.readLong(0)
        while(true) {
            while(value == lastValue) {
                java.lang.Thread.onSpinWait();
                value = readBuf.readLong(0)
            }
            suspendPoint.resume( value )
        }
    }
}
```
`spinLoop()` is called from `main()`, and is the "real" infinite loop. It is not called from a `suspend` context. It's just normal code. So it spins until the value changes, then resumes the coroutine code, that will return when the coroutine blocks again. It's effectivly doing a callback for you. But you get to write your code as if it isn't. If you are familuar with kotlin, this might not look strange. None of this code was witting w/ the `kotlinx.coroutine` library. Just the primitive support from the compiler.

All the async version were wirtten the same. The async code suspends while the vent loop spins on memory. When the async code resumes w/ the new value, it writes it.

## Testing enviroment

I used the machines I have at home for this. One is an `Intel i7-8700K`, the other is a newer `AMD Ryzen 5 5600G`. All the tests were run on bare metal. VMs might be fine for this kind of test though. Both CPUs have 6 core, 2 CPUs per core. I rant a kernel w/ the `ioslcpu` param to isolate cores 5 & 6 (CPUs 4,5,10 & 11). I ran the client pinned to 4, and the server to 5. the AMD is headless, so easy to strip. The Intel is my dev box, so I'd boot to multi-user when running tests. Both are Ubuntu 20.04.
Each test did a 3 second warm up, then ran for 30 seconds. 1K of samples are taken from each run. You can reach about criterion's sampling methodology if you are curious about that. It also has great graphing out of the box, but I love R's ggplot. JVM was Graal 11.0.12

## TLDR; Results
[!Intel](graphs/intel.png)
[!Amd](graphs/amd.png)



