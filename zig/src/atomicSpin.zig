
const std = @import("std");
const utils = @import("common.zig");



pub fn main() anyerror!void {

    const ptrs = try utils.doSetup();

    runLoop( ptrs.clientPtr, ptrs.serverPtr );

}

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
