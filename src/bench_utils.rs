use crate::SAMPLE_SIZE;
use criterion::{BatchSize, Criterion};
use rand::RngCore;
use std::time::Duration;

lazy_static! {
    pub static ref JAVA_OPTS: Vec<&'static str> = vec![
        "--illegal-access=permit",
        "--add-exports",
        "java.base/jdk.internal.ref=ALL-UNNAMED",
        "-server",
        "--illegal-access=permit",
        // "-XX:+PrintCompilation", // un-comment to see when the JIT decides to compile.

    ];
}
pub fn launch_local(cmd: &str, params: &Vec<&str>) -> std::process::Child {
    let mut process = std::process::Command::new(cmd);

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
    let mut process = std::process::Command::new("java");

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
pub fn run_bench<F>(c: &mut Criterion, group_name: &str, bench_name: &str, mut bench_fn: F)
where
    F: FnMut(u64),
{
    let mut group = c.benchmark_group(group_name);
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(SAMPLE_SIZE);
    group.bench_function(bench_name, |b| {
        b.iter_batched(
            || rand::thread_rng().next_u64(),
            |payload| bench_fn(payload),
            BatchSize::SmallInput,
        )
    });
    group.finish();
}
