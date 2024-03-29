use async_bench::atomic_spin::MappedAtomics;
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
    callback: Option<&'a dyn Fn(&mut T, u64)>,
    atomics: &'a MappedAtomics,
}

impl<'a, T> EventLoop<'a, T> {

    fn set_callback(&mut self, cb:&'a impl Fn(&mut T, u64) ) {
        self.callback = Some(cb);
    }

    fn run(&mut self) {
        let mut last_value : u64 = 0;
        loop {
            last_value = self.atomics.server_spin_until_change(last_value);
            if let Some(cb) = &self.callback {
                cb( &mut self.context, last_value );
            }
        }
    }
}

fn main() -> io::Result<()> {

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


    // let func = &Worker::do_work;
    // let th : FnMut(&mut Worker, u64) = func;
    ev.set_callback( &Worker::do_work );

    // th(&wk,99);

    ev.run();

    // wk.someState = 200;
    server.client_run_once(99);
    Ok(())
}
