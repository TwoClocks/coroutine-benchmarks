use async_bench::atomic_spin::MappedAtomics;
use std::io;

fn main() -> io::Result<()> {

    let server = MappedAtomics::new(false);

    println!("\nstarting server");
    server.do_server_loop();

    Ok(())
}
