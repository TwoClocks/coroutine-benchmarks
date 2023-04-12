
const std = @import("std");
const utils = @import("common.zig");



const EventLoop = struct {
    const callbackWrap = struct {
        callback: fn(*anyopaque, u64) void,
        context: *anyopaque,

        fn doCall(self: callbackWrap, value: u64) void {
            (self.callback)(self.context, value);
        }
    };

    callback :?callbackWrap = null,
    clientPtr : *const u64,

    pub fn setCallback(
        self:*EventLoop,
        self_ptr:anytype,
        comptime F:fn(@TypeOf(self_ptr),u64)void
    ) void {
        self.callback = .{
            .context = @ptrCast(*anyopaque, self_ptr),
            .callback = @ptrCast(fn(*anyopaque, u64) void, F),
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
        @atomicStore(u64, self.serverPtr, value, .Monotonic );
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

