
const std = @import("std");
const utils = @import("common.zig");



pub fn main() anyerror!void {

    const ptrs = try utils.doSetup();

    runLoop( ptrs.clientPtr, ptrs.serverPtr );

}

fn runLoop( clientPtr : *const u64, serverPtr : *u64 ) void {
    
    var lastValue : u64 = 0;

    while(true) {
        lastValue = utils.spinUntilChange( clientPtr, lastValue );
        @atomicStore(u64, serverPtr, lastValue, std.builtin.AtomicOrder.Monotonic );
    }
}

