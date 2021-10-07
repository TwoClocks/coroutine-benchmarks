use async_bench::atomic_spin::MappedAtomics;
use core_affinity::CoreId;
use std::cell::RefCell;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// I needed a place both the async code, the Future
/// and the eventLoop can mutate and share information
/// between them.
/// it can't be in the future, because when the future is .await'd
/// the mutable reference to the Future is taken. So it can't be
/// mutated while suspended.
/// It can't be in the eventloop, because the event loops
/// is mutating itself while it's looping. Same problem.
/// So it's here, and it's passed to both the Future
/// and the EventLoop.
struct RuntimeState {
    atomics: MappedAtomics,
    waker: Option<Waker>,
    /// what was the last value the client sent us.
    /// we spin on client memory until the memory
    /// is no longer this value, then set this value
    /// to what we read.
    last_moved: Option<u64>,
    /// this is the last client value that was read,
    /// and needs to be copied to the server memory.
    /// We could have just used one var for both of these
    /// but it makes the code a little more confusing
    /// and it doesn't change anything.
    to_move: Option<u64>,
}

/// the main server loop. Async this time.
/// it suspends until the client memory has changed.
/// it then copies the client memory to the server memory.
async fn async_server_loop(state: Rc<RefCell<RuntimeState>>) {
    let mut fut = SpinFuturre::new(Rc::clone(&state));
    let mut value: u64 = 0;

    loop {
        // wait fot the client memory to change.
        // this is a suspending call.
        value = fut.wait_on_next_value(value).await;

        // write the new value to the server memory.
        state
            .borrow()
            .atomics
            .server_write
            .store(value, Ordering::Relaxed);
    }
}

fn main() -> io::Result<()> {
    // println!("main running");
    let cpu_num: usize = std::env::args()
        .nth(1)
        .expect("pass CPU # to ping to")
        .parse()
        .expect("Can't parse passed CPU # as a number");

    core_affinity::set_for_current(CoreId { id: cpu_num });

    let state = Rc::new(RefCell::new(RuntimeState {
        atomics: MappedAtomics::new(false),
        waker: None,
        to_move: None,
        last_moved: None,
    }));

    let mut spin_code = async_server_loop(Rc::clone(&state));

    let mut task = Task::init(&mut spin_code);
    task.advance(); // start the async code running.

    // run forever.
    run_event_loop(Rc::clone(&state));

    #[allow(unreachable_code)]
    Ok(())
}

// this loop assumes it's starting state is that
// the async client loop is already running, and it's
// already suspended waiting for the client memory to change.
fn run_event_loop(state: Rc<RefCell<RuntimeState>>) {
    loop {
        // get the last client value. We'll spin until
        // the memory changes from this value.
        let last = state.borrow_mut().last_moved.take().unwrap();

        // this is the spin loop.
        let next = state.borrow().atomics.server_spin_until_change(last);

        // record to new value for the Future to pick up
        // on next poll
        state.borrow_mut().to_move = Some(next);

        // poll the future.
        let wk = state.borrow_mut().waker.take().unwrap();
        // when we return from this, the async code will be locked
        // on the next iteration.
        wk.wake();
    }
}

/// this is the Future we'll use
/// as a suspend point.
struct SpinFuturre {
    state: Rc<RefCell<RuntimeState>>,
}

impl SpinFuturre {
    fn new(state: Rc<RefCell<RuntimeState>>) -> SpinFuturre {
        SpinFuturre { state }
    }

    /// suspend until the memory changes.
    async fn wait_on_next_value(&mut self, last_value: u64) -> u64 {
        self.state.borrow_mut().last_moved = Some(last_value);
        self.await
    }
}


impl Future for SpinFuturre {
    type Output = u64;

    /// do work.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        let mut state = self.state.borrow_mut();
        // if to_move is not None, we're done. Return the value in it.
        // else, store the waker for the event loop to use,
        // and return Pending
        match state.to_move.take() {
            Some(nx) => Poll::Ready(nx),
            None => {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

/// the thing that starts the task.
/// Because the waker just calls poll()
/// the waker needs to contain everything
/// needed to call poll()
/// this means it needs both the pin'd
/// top-level Future and itself.
/// I do not know how to get around the
/// waker containing itself.
/// so the creation is super janky.
struct Task<'a> {
    code: Pin<&'a mut dyn Future<Output = ()>>,
    waker: Waker,
}

impl<'a> Task<'_> {

    // create a waker for a block of async code.
    pub fn init(block: &'a mut dyn Future<Output = ()>) -> Task {
        unsafe {
            // first create a waker w/ a null pointer. We'll overwrite it in a moment.
            let w = Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)); // set a waker w/ a bad pointer.
            // pin the async code to the stack.
            let mut ret = Task {
                code: Pin::new_unchecked(block), // put it on the stack.
                waker: w,
            };
            // now replace the waker, with a waker that points to it's own struct.
            // I assume there is another way to do this, but It's not obvious to me.
            ret.waker = {
                let ptr = &ret as *const Task as *const ();
                let raw_waker = RawWaker::new(ptr, &VTABLE);
                Waker::from_raw(raw_waker)
            };
            ret
        }
    }

    // Waker.wake() just calls this.
    // it's also called once from main() to
    // start the async task.
    fn advance(&mut self) {
        let mut cx = Context::from_waker(&self.waker);
        // keep kicking it until it's suspended.
        while self.code.as_mut().poll(&mut cx).is_ready() {}
    }
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    |s| run_clone(s),
    |s| run_wake(s),
    |s| run_wake(s),
    |_| {}, // dromp on a reference doesn't do anyting.
);

// decode the pointer as a
// task, and call advance.
fn run_wake(s: *const ()) {
    // println!("waker.wake");
    let r = unsafe {
        // can't cast to mutable w/o transmute
        std::mem::transmute::<*const (), &mut Task>(s)
    };
    r.advance();
}

fn run_clone(s: *const ()) -> RawWaker {
    RawWaker::new(s, &VTABLE)
}
