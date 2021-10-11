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
        value = utils.spinUntilChange( clientPtr, value );
        ev.putResult( serverPtr, value );
    }
}

const EventLoop = struct {
    value : u64 = 0,
    writePtr : *u64 = undefined,
    suspend_point : anyframe = undefined,

    pub fn putResult(self:*EventLoop, writePtr:*u64,value : u64 ) void {
        self.writePtr = writePtr;
        self.value = value;
        suspend {
            self.suspend_point = @frame();
        }
    }


    pub fn run( self:*EventLoop ) void {
        while(true) {
            @atomicStore(u64, self.writePtr, self.value, std.builtin.AtomicOrder.Monotonic );

            resume self.suspend_point;
        }
    }
};

