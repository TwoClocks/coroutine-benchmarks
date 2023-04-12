use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use std::io;
use async_bench::atomic_spin::MappedAtomics;
use async_bench::async_impl::{RuntimeState, SpinFuture, Task};


// this other way around. we spin in the async code
// and write the value in the event_loop. This way
// we'll be timing suspend instead of resume.
async fn async_loop_suspend(state: Rc<RefCell<RuntimeState>>) {
    let mut fut = SpinFuture::new(Rc::clone(&state));
    let mut value: u64 = 0;

    loop {
        // wait for the memory to change.
        value = state.borrow().atomics.server_spin_until_change(value);

        // tell the event loop to write the value.
        fut.suspend_to_eventloop(value).await;

    }
}

fn main() -> io::Result<()> {
    
    let state = Rc::new(RefCell::new(
        RuntimeState::new(
            MappedAtomics::new( false )
        )
    ));

    let spin_code = async_loop_suspend(Rc::clone(&state));

    // start the async code running.
    // I don't think I need the return value for anything.
    let mut task = Task::init(spin_code);
    task.advance();

    // run forever.
    event_loop_suspend(Rc::clone(&state));

    #[allow(unreachable_code)]
    Ok(())
}

fn event_loop_suspend(state: Rc<RefCell<RuntimeState>>) {
    loop {

        let wk = {
            let mut s = state.borrow_mut();
            // write the value.
            let v = s.to_event_loop.unwrap();
            s.atomics.server_write.store(v, Ordering::Relaxed);
            s.to_async_loop = Some(v);

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

