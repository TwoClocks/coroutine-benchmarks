use async_bench::atomic_spin::MappedAtomics;
use core_affinity::CoreId;
use std::io;

fn main() -> io::Result<()> {
    let cpu_num: usize = std::env::args()
        .nth(1)
        .expect("pass CPU # to ping to")
        .parse()
        .expect("Can't parse passed CPU # as a number");

    core_affinity::set_for_current(CoreId { id: cpu_num });

    let server = MappedAtomics::new(false);

    println!("\nstarting server");
    server.do_server_loop();

    Ok(())
}
