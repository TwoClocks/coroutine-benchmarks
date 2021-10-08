use criterion::*;

use core_affinity::CoreId;
use async_bench::atomic_spin::MappedAtomics;
use async_bench::{CLIENT_CPU, SERVER_CPU};

fn rust_bench(c: &mut Criterion) {
    // map memory
    let client = MappedAtomics::new(true);

    core_affinity::set_for_current(CoreId { id: CLIENT_CPU });

    let mut child = async_bench::bench_utils::launch_local(
        "target/release/atomic_spin_server",
        vec![SERVER_CPU].as_ref(),
    );

    async_bench::bench_utils::run_bench(c, "atomic_spin", "rust_atomic", &client );

    client.close();
    child.kill().expect("error killing server process");
}

fn rust_async_bench(c: &mut Criterion) {
    // map memory
    let mut client = MappedAtomics::new(true);

    core_affinity::set_for_current(CoreId { id: CLIENT_CPU });

    let mut child = async_bench::bench_utils::launch_local(
        "target/release/atomic_async_server",
        vec![SERVER_CPU].as_ref(),
    );

    async_bench::bench_utils::run_bench(c, "atomic_spin", "rust_async", &client);

    client.close();
    child.kill().expect("error killing server process");
}

fn zig_bench(c: &mut Criterion) {
    // map memory
    let mut client = MappedAtomics::new(true);

    core_affinity::set_for_current(CoreId { id: CLIENT_CPU });

    let mut child =
        async_bench::bench_utils::launch_local("zig/zig-out/bin/atomicSpin", vec![SERVER_CPU].as_ref());

    async_bench::bench_utils::run_bench(c, "atomic_spin", "zig_atomic", &client);

    client.close();
    child.kill().expect("error killing server process");
}

fn zig_async_bench(c: &mut Criterion) {
    // map memory
    let mut client = MappedAtomics::new(true);

    core_affinity::set_for_current(CoreId { id: CLIENT_CPU });

    let mut child = async_bench::bench_utils::launch_local(
        "zig/zig-out/bin/atomicAsyncSpin",
        vec![SERVER_CPU].as_ref(),
    );

    async_bench::bench_utils::run_bench(c, "atomic_spin", "zig_async", &client);

    client.close();
    child.kill().expect("error killing server process");
}

fn kotlin_bench(c: &mut Criterion) {
    // map memory
    let mut client = MappedAtomics::new(true);

    core_affinity::set_for_current(CoreId { id: CLIENT_CPU });

    let mut child = async_bench::bench_utils::launch_local_java(
        "kotlin/app/build/libs/app-all.jar",
        "kotlin_servers.AtomicSpinServerKt",
        Some(async_bench::bench_utils::JAVA_OPTS.as_ref()),
        vec![SERVER_CPU].as_ref(),
    );

    async_bench::bench_utils::run_bench(c, "atomic_spin", "kotlin_atomic", &client);

    client.close();
    child.kill().expect("error killing server process");
}

fn kotlin_async_bench(c: &mut Criterion) {
    let mut client = MappedAtomics::new(true);

    core_affinity::set_for_current(CoreId { id: CLIENT_CPU });

    let mut child = async_bench::bench_utils::launch_local_java(
        "kotlin/app/build/libs/app-all.jar",
        "kotlin_servers.AsyncSpinServerKt",
        Some(async_bench::bench_utils::JAVA_OPTS.as_ref()),
        vec![SERVER_CPU].as_ref(),
    );

    async_bench::bench_utils::run_bench(c, "atomic_spin", "kotlin_async", &client);

    client.close();
    child.kill().expect("error killing server process");
}

criterion_group!(
    benches,
    rust_bench,
    rust_async_bench,
    zig_bench,
    zig_async_bench,
    kotlin_bench,
    kotlin_async_bench,
);
// criterion_group!(benches, kotlin_bench);
criterion_main!(benches);
