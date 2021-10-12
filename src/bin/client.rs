use async_bench::atomic_spin::MappedAtomics;
use core_affinity::CoreId;
use std::io;
use rand::RngCore;


///
/// Not used by the tests. Just a stant-alone client that does no
/// benchmarking. Useful if you want to run a debug build of a benchmark
/// for debugging.
fn main() -> io::Result<()> {

    let cpu_num: usize = std::env::args()
        .nth(1)
        .expect("pass CPU # to ping to")
        .parse()
        .expect("Can't parse passed CPU # as a number");

    core_affinity::set_for_current(CoreId { id: cpu_num });

    let server = MappedAtomics::new(true);

    loop {
        let val = rand::thread_rng().next_u64();
        eprintln!("setting value {}",val);
        server.client_run_once(val);
    }
    #[allow(unreachable_code)]
    Ok(())
}
