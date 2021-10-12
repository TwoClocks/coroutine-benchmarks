const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    // const mode = b.standardReleaseOptions();

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


    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
