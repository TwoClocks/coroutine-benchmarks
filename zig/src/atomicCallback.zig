
const std = @import("std");
const utils = @import("common.zig");

// #
// # libuv / Calloop / Stakker
// #

const EventLoop = struct {
    const cbWrap = struct {
        const callType = fn(*c_void, u64) void;
        callback : *const c_void,
        context : *c_void,

        fn doCall(self:*const cbWrap, value:u64) void {
            const call_raw = @alignCast(@alignOf(callType), self.callback);
            const call_ptr = @ptrCast(callType, call_raw);
            @call(.{}, call_ptr, .{self.context, value});

        }
    };
    callback : ?cbWrap = null,
    clientPtr : *const u64,

    pub fn setCallback(
        self:*EventLoop,
        C:anytype,
        comptime F:fn(@TypeOf(C),u64)void
    ) void {
        self.callback = cbWrap {
            .callback = @ptrCast(*const c_void, F),
            .context = @ptrCast(*c_void, C),
        };
    }

    pub fn run(self:*EventLoop ) void {
        var last_value : u64 = 0;
        while(true) {
            last_value = utils.spinUntilChange(self.clientPtr, last_value);
            if( self.callback ) |cb| {
              cb.doCall(last_value);
            }
        }
    }
};


const SomeWorker = struct {
    someState : u64 = 0,
    serverPtr : *u64,

    pub fn doWork(self:*SomeWorker, value:u64) void {
        @atomicStore(u64, self.serverPtr, value, std.builtin.AtomicOrder.Monotonic );
        self.someState = value;
    }
};


pub fn main() anyerror!void {

    const ptrs = try utils.doSetup();

    var ev = EventLoop{
        .clientPtr = ptrs.clientPtr
    };

    var worker = SomeWorker{
        .serverPtr = ptrs.serverPtr
    };

    ev.setCallback( &worker, SomeWorker.doWork );

    ev.run();

}

