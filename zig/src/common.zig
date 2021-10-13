const c = @cImport({
    @cDefine("_GNU_SOURCE",{});

    @cInclude("sys/mman.h");
    @cInclude("sys/stat.h");
    @cInclude("fcntl.h");
    @cInclude("sys/types.h");
    @cInclude("unistd.h");
    @cInclude("errno.h");
    @cInclude("sched.h");
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

// pub fn SuspendHelper(comptime T:type ) type {
//     return struct {
//         const Self = @This();
//         value: T,
//         suspend_context : ?anyframe = null,

//         pub fn suspendMe(self:*Self) T {
//             if(self.suspend_context) |_| {
//                 std.debug.panic("suspend context already exists. re-enternat code. die\n",.{});
//             } else {
//                 suspend {
//                     self.suspend_context = @frame(); // this is how you suspend in zig.
//                 }
//             }
//             return self.value;
//         }

//         pub fn resumeMe(self:*Self, value:T) void {
//             if(self.suspend_context) |w| {
//                 self.value = value;
//                 const tmp = w;
//                 self.suspend_context = null;
//                 resume tmp;
//             } else {
//                 std.debug.print("suspend : resume called w/ no frame.",.{});
//             }
//         }
    
//     };
// }



const SetupError = error {
    C_Func_Error
};


const SetupReturn = struct {
    clientPtr : *const u64,
    serverPtr : *u64,
};

pub fn doSetup() anyerror!SetupReturn {
    const allocator = std.heap.c_allocator;

    var args = std.process.args();
    
    // don't care about my exe name.
    _ = args.skip();

    const tmp = try args.next(allocator).?;
    const cpuPinNum = try std.fmt.parseInt( u6, tmp, 10 );
    // std.log.err("setting cpu num to {}\n",.{cpuPinNum});


    var memHandle : c_int = c.shm_open(
        "/spinnmem",
        c.O_RDWR,
        c.S_IRUSR | c.S_IWUSR | c.S_IRGRP | c.S_IWGRP
    );
    if( memHandle < 0 ) {
        std.log.err("can't open shm location. error code {}",.{std.os.system._errno().*});
        return SetupError.C_Func_Error;
    }

    if( c.ftruncate(memHandle, 4096 ) < 0 ) return SetupError.C_Func_Error;


    const memPtr = try std.os.mmap(
        null,
        4096,
        c.PROT_READ | c.PROT_WRITE,
        c.MAP_SHARED,
        memHandle,
        0
    );

    // zig can't parse any of the CPU_ZERO or CPU_SET macors in
    // sched.h and cpu-set.h. So gotta do it by hand.
    // I *think* it's just a bit mask. 1 bit for every cpu.
    // but not 100% sure.
    var cpuMask : c.cpu_set_t = std.mem.zeroes( c.cpu_set_t);
    
    const cpuSetSize = @sizeOf(c.cpu_set_t);

    cpuMask.__bits[0] = @as(c_ulong,1) << cpuPinNum;

    if( c.sched_setaffinity(0,cpuSetSize,&cpuMask)  < 0 ) {
        std.log.err("can't set cpu affinity. error code {}",.{std.os.system._errno().*});
        return SetupError.C_Func_Error;
    }

    return SetupReturn{
        .clientPtr = std.mem.bytesAsValue( u64, memPtr[0..8]),
        .serverPtr = std.mem.bytesAsValue( u64, memPtr[2048..(2048+8)])
    };

    

}
