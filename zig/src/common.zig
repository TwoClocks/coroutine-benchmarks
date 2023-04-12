const c = @cImport({
    @cInclude("fcntl.h");
    @cInclude("sys/mman.h");
});

const std = @import("std");


pub fn spinUntilChange( spinPtr:*const u64, lastValue:u64) callconv(.Inline) u64 {

    var newValue = lastValue;

    while( newValue == lastValue ) {
        std.atomic.spinLoopHint();
        newValue = @atomicLoad(u64, spinPtr, std.builtin.AtomicOrder.Monotonic );
    }
    return newValue;
}


const SetupError = error {
    C_Func_Error
};

const SetupReturn = struct {
    clientPtr : *const u64,
    serverPtr : *u64,
};

// export as a C lib for the JVM code to 
// map the shared memory
export fn mapSetup() *const anyopaque {
    const mem = doSetup() catch  {
        std.debug.panic("Can't setup shared memory. Fail",.{});
    };
    return mem.clientPtr;
}

pub fn doSetup() anyerror!SetupReturn {

    var memHandle : c_int = std.c.shm_open(
        "/spinnmem",
        c.O_RDWR,
        c.S_IRUSR | c.S_IWUSR | c.S_IRGRP | c.S_IWGRP
    );
    if( memHandle < 0 ) {
        std.log.err("can't open shm location. error code {}",.{std.os.system._errno().*});
        return SetupError.C_Func_Error;
    }

    if( std.c.ftruncate(memHandle, 4096 ) < 0 ) return SetupError.C_Func_Error;


    const memPtr = try std.os.mmap(
        null,
        4096,
        c.PROT_READ | c.PROT_WRITE,
        c.MAP_SHARED,
        memHandle,
        0
    );

    return SetupReturn{
        .clientPtr = std.mem.bytesAsValue( u64, memPtr[0..8]),
        .serverPtr = std.mem.bytesAsValue( u64, memPtr[2048..(2048+8)])
    };

}
