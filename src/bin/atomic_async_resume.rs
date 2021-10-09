use async_bench::async_impl::{RuntimeState, Task, SpinFuture};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::io;
use core_affinity::CoreId;
use async_bench::atomic_spin::MappedAtomics;

/// the main server loop. Async this time.
/// it suspends until the client memory has changed.
/// it then copies the client memory to the server memory.
async fn async_loop_resume(state: Rc<RefCell<RuntimeState>>) {
    let mut fut = SpinFuture::new(Rc::clone(&state));
    let mut value: u64 = 0;

    loop {
        // wait fot the client memory to change.
        // this is a suspending call.
        value = fut.suspend_to_eventloop(value).await;

        // write the new value to the server memory.
        state
            .borrow()
            .atomics
            .server_write
            .store(value, Ordering::Relaxed);
    }
}

fn main() -> io::Result<()> {


    let cpu_num: usize = std::env::args()
        .nth(1)
        .expect("pass CPU # to ping to")
        .parse()
        .expect("Can't parse passed CPU # as a number");

    core_affinity::set_for_current(CoreId { id: cpu_num });

    let state = Rc::new(
        RefCell::new(
            RuntimeState::new(
                MappedAtomics::new(false)
            )));

    let spin_code = async_loop_resume(Rc::clone(&state));

    let mut task = Task::init(spin_code);
    task.advance();

    // run forever.
    event_loop_resume(Rc::clone(&state));

    #[allow(unreachable_code)]
    Ok(())
}

// this loop assumes it's starting state is that
// the async client loop is already running, and it's
// already suspended waiting for the client memory to change.
fn event_loop_resume(state: Rc<RefCell<RuntimeState>>) {
    loop {


        // can't keep the mut barrow outstanding when we call wake()
        let wk = {
            let mut s = state.borrow_mut();


            // get the last client value. We'll spin until
            // the memory changes from this value.
            let last = s.to_event_loop.take().unwrap();

            // this is the spin loop.
            let next = s.atomics.server_spin_until_change(last);

            // record to new value for the Future to pick up
            // on next poll
            s.to_async_loop = Some(next);

            // poll the future.
            s.waker.take()
        };
        assert!(wk.is_some());
        if let Some(w) = wk {
            // when we return from this, the async code will be locked
            // on the next iteration.
            w.wake();
        }
    }
}

