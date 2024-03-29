use crate::{SAMPLE_SIZE, SERVER_CPU};
use criterion::{BatchSize, Criterion};
use rand::RngCore;
use crate::atomic_spin::MappedAtomics;
use thread_priority::ThreadPriority;

lazy_static! {
    // Must have java 19
    pub static ref JAVA_OPTS: Vec<&'static str> = vec![
        "-server",
        "-Djava.library.path=zig/zig-out/lib",  // JVM used zig for shm_open call
        "--enable-native-access=ALL-UNNAMED",
        // "-XX:+PrintCompilation", // un-comment to see when the JIT decides to compile.

    ];
}
pub fn launch_local(cmd: &str, params: &Vec<&str>) -> std::process::Child {
    let mut process = std::process::Command::new("nice");
    process.arg("-n").arg("-20").arg("taskset").arg("-c").arg(SERVER_CPU).arg(cmd);

    for prm in params.iter() {
        process.arg(prm);
    }
    process
        .spawn()
        .expect(format!("Can't spawn child process {}", cmd).as_str())
}

pub fn launch_local_java(
    jar_file: &str,
    run_class: &str,
    java_opts: Option<&Vec<&str>>,
    program_args: &Vec<&str>,
) -> std::process::Child {
    let mut process = std::process::Command::new("nice");
    process.arg("-n").arg("-20").arg("taskset").arg("-c").arg(SERVER_CPU).arg("java");

    if let Some(j_opts) = java_opts {
        for opt in j_opts.iter() {
            process.arg(opt);
        }
    }

    process.arg("-cp").arg(jar_file).arg(run_class);

    for prm in program_args.iter() {
        process.arg(prm);
    }

    process.spawn().expect("can't start java process")
}

/// some boilerplate code pulled out into a function.
pub fn run_bench(c: &mut Criterion, group_name: &str, bench_name: &str, client:&MappedAtomics )
{
    ThreadPriority::Max.set_for_current().unwrap();

    // run the client code once, just to make sure the server is up
    // before we start. Sometimes the java code can take a little
    // while to startup... bless it's little heart.
    client.client_run_once(12345678 );

    // let thid = std::thread::current().id();
    let mut group = c.benchmark_group(group_name);
    // group.warm_up_time( WARMUP_TIME );
    // group.measurement_time(RUN_TIME);
    group.sample_size(SAMPLE_SIZE);
    group.bench_function(bench_name, |b| {
        b.iter_batched(
            || {
                // convince myself that the time to gen the rnd
                // isn't part of the timing test. un-comment
                // out next like and see if the benchmark
                // results change
                // std::thread::sleep( std::time::Duration::from_millis(1));
                rand::thread_rng().next_u64()
            },
            |payload| {
                // assert_eq!(thid, std::thread::current().id());
                client.client_run_once(payload)
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}
