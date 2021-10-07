const std = @import("std");
const utils = @import("common.zig");


// pub fn main() void {}
pub fn main() anyerror!void {

    // std.debug.print("{}",.{@TypeOf(spinWait)});

    const ptrs = try utils.doSetup();

    var eventLoop = EventLoop{};

    _ = async asyncLoop(ptrs.clientPtr, ptrs.serverPtr, &eventLoop);

    eventLoop.run();

}


// this is the async loop
fn asyncLoop( clientPtr : *const u64, serverPtr : *u64, ev:*EventLoop) void {
    var value : u64 = 0;
    while(true) {
        value = await async ev.getNextValue(clientPtr,value);
        @atomicStore(u64, serverPtr, value, std.builtin.AtomicOrder.Monotonic );
    }
}

const EventLoop = struct {
    last_value : u64 = 0,
    readPtr : *const u64 = undefined,
    spinWait : utils.SuspendHelper(u64) = .{.value=0},

    pub fn getNextValue(self:*EventLoop, readPtr:*const u64,last : u64 ) u64 {
        self.readPtr = readPtr;
        self.last_value = last;
        return self.spinWait.suspendMe();
    }


    pub fn run( self:*EventLoop ) void {
        while(true) {
            const next = spinUntilChange(self.readPtr,self.last_value);
            self.spinWait.resumeMe( next );
        }
    }
};

fn spinUntilChange( spinPtr:*const u64, lastValue:u64) callconv(.Inline) u64 {

    var newValue = @atomicLoad(u64, spinPtr, std.builtin.AtomicOrder.Monotonic );

    while( newValue == lastValue ) {
        std.atomic.spinLoopHint();
        newValue = @atomicLoad(u64, spinPtr, std.builtin.AtomicOrder.Monotonic );
    }
    return newValue;
}
