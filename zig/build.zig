const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.build.Builder) void {

    // this code depends on zig version 0.10.1
    const zig_version = builtin.zig_version;
    if (builtin.zig_version.major < 0 or zig_version.minor < 10 or zig_version.patch < 1) {
        std.debug.err("This code depends on zig version 0.10.1 or later");
        return;
    }

    b.use_stage1=true;

    const exe = b.addExecutable("atomicSpin", "src/atomicSpin.zig");
    exe.single_threaded = true;
    exe.setBuildMode(std.builtin.Mode.ReleaseFast);
    // exe.setBuildMode(std.builtin.Mode.Debug);
    exe.linkLibC();
    exe.install();

    const exe2 = b.addExecutable("atomicAsyncResume", "src/atomicAsyncResume.zig");
    exe2.single_threaded = true;
    exe2.setBuildMode(std.builtin.Mode.ReleaseFast);
    // exe.setBuildMode(std.builtin.Mode.Debug);
    exe2.linkLibC();
    exe2.install();

    const exe3 = b.addExecutable("atomicAsyncSuspend", "src/atomicAsyncSuspend.zig");
    exe3.single_threaded = true;
    exe3.setBuildMode(std.builtin.Mode.ReleaseFast);
    // exe.setBuildMode(std.builtin.Mode.Debug);
    exe3.linkLibC();
    exe3.install();

    const exe4 = b.addExecutable("atomicCallback", "src/atomicCallback.zig");
    exe4.single_threaded = true;
    exe4.setBuildMode(std.builtin.Mode.ReleaseFast);
    // exe.setBuildMode(std.builtin.Mode.Debug);
    exe4.linkLibC();
    exe4.install();

    // make a .so file for the JVM code for
    // maping memory via shm_open
    const lib = b.addSharedLibrary("zigmap", "src/common.zig", b.version(0,0,1));
    lib.setBuildMode(std.builtin.Mode.ReleaseFast);
    lib.linkLibC();
    lib.install();

}
