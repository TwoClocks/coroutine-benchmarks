use async_bench::atomic_spin::MappedAtomics;
use core_affinity::CoreId;
use std::io;
use std::sync::atomic::Ordering;



struct Worker<'a> {
    atomcis: &'a MappedAtomics,
    some_state: u64,
}

impl<'a> Worker<'a> {
    fn do_work(&mut self, value:u64) {
        self.atomcis.server_write.store( value, Ordering::Relaxed);
        self.some_state = value;
    }
}


struct EventLoop<'a, T> {
    context : T,
    callback: Option<&'a mut dyn FnMut(&mut T, u64)>,
    atomics: &'a MappedAtomics,
}

impl<'a, T> EventLoop<'a, T> {
    fn set_callback(&mut self, cb: &'a mut dyn FnMut(&mut T, u64) ) {
        self.callback = Some(cb);
    }
    fn run(&mut self) {
        let mut last_value : u64 = 0;
        loop {
            last_value = self.atomics.server_spin_until_change(last_value);
            if let Some(cb) = &mut self.callback {
                cb( &mut self.context, last_value );
            }
        }
    }
}

fn main() -> io::Result<()> {
    let cpu_num: usize = std::env::args()
        .nth(1)
        .expect("pass CPU # to ping to")
        .parse()
        .expect("Can't parse passed CPU # as a number");

    core_affinity::set_for_current(CoreId { id: cpu_num });

    let server = MappedAtomics::new(false);

    let wk = Worker{
        atomcis : &server,
        some_state: 0
    };

    let mut ev = EventLoop {
        context : wk,
        callback : None,
        atomics : &server
    };


    let func = &mut Worker::do_work;
    ev.set_callback( func );

    ev.run();

    // wk.someState = 200;
    server.client_run_once(99);
    Ok(())
}
