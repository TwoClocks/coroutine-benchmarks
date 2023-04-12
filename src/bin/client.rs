use async_bench::atomic_spin::MappedAtomics;
use std::io;
use rand::RngCore;


///
/// Not used by the tests. Just a stant-alone client that does no
/// benchmarking. Useful if you want to run a debug build of a benchmark
/// for debugging.
fn main() -> io::Result<()> {

    let server = MappedAtomics::new(true);

    loop {
        let val = rand::thread_rng().next_u64();
        eprintln!("setting value {}",val);
        server.client_run_once(val);
    }
    #[allow(unreachable_code)]
    Ok(())
}
