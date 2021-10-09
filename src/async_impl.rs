use std::task::{RawWaker, RawWakerVTable, Context, Waker, Poll};
use std::pin::Pin;
use std::future::Future;
use std::rc::Rc;
use std::cell::RefCell;
use crate::atomic_spin::MappedAtomics;


/// unlike zig and kotlin
/// the async loop and the event loop
/// can't communicate directly. Everything is via
/// the Future. But since await'ign on the future
/// takes the mut ref on the Future, and the event_loop spin code also
/// takes the mut ref on the event loop, there must be some 3rd place
/// the Future and EventLoop all can reach mutably.
/// this is it.
pub struct RuntimeState {
    pub atomics: MappedAtomics,
    pub waker: Option<Waker>,

    /// The values that the async loop
    /// want to tell the event loop about
    pub to_event_loop: Option<u64>,

    /// the value the event loop
    /// wants to tell the async
    /// loop about
    pub to_async_loop: Option<u64>,
}

impl RuntimeState {
    pub fn new(atomics:MappedAtomics) -> RuntimeState {
        RuntimeState{
            atomics,
            waker : None,
            to_event_loop : None,
            to_async_loop : None,
        }
    }
}


/// this is the Future we'll use
/// as a suspend point.
pub struct SpinFuture {
    state: Rc<RefCell<RuntimeState>>,
}

impl SpinFuture {
    pub fn new(state: Rc<RefCell<RuntimeState>>) -> SpinFuture {
        SpinFuture { state }
    }

    pub async fn suspend_to_eventloop(&mut self, to_event: u64) -> u64 {
        self.state.borrow_mut().to_event_loop = Some(to_event);
        self.await
    }
}


impl Future for SpinFuture {
    type Output = u64;

    /// do work.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.borrow_mut();
        // if to_move is not None, we're done. Return the value in it.
        // else, store the waker for the event loop to use,
        // and return Pending
        match state.to_async_loop.take() {
            Some(nx) => {
                Poll::Ready(nx)
            },
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
/// waker containing a reference to the
/// struct it's housed in.
/// So the creation is super janky.
pub struct Task<F> {
    code: Pin<Box<F>>,
    waker: Waker,
}

impl<F> Task<F>
    where
        F : Future<Output = ()>
{

    // create a waker for a block of async code.
    pub fn init(block: F) -> Task<F> {
        unsafe {
            let mut task = Task {
                code: Box::pin(block),
                // first create a waker w/ a null pointer. We'll overwrite it in a moment.
                waker: Waker::from_raw(RawWaker::new(std::ptr::null(), &Task::<F>::VTABLE)), // set a waker w/ a bad pointer.
            };

            // now replace the waker, with a waker that points to it's own struct.
            task.waker = {
                let ptr = &task as *const Task<F> as *const ();
                let raw_waker = RawWaker::new(ptr, &Task::<F>::VTABLE);
                Waker::from_raw(raw_waker)
            };
            task
        }
    }

    // Waker.wake() just calls this.
    // it's also called once from main() to
    // start the async task.
    pub fn advance(&mut self) {

        let mut cx = Context::from_waker(&self.waker);
        // keep kicking it until it's suspended.
        while self.code.as_mut().poll(&mut cx).is_ready() {}
    }

    const VTABLE: RawWakerVTable = RawWakerVTable::new(
        |s| Task::<F>::run_clone(s),
        |s| Task::<F>::run_wake(s),
        |s| Task::<F>::run_wake(s),
        |_| {}, // droping on a reference doesn't do anyting.
    );

    // decode the pointer as a
    // task, and call advance.
    pub fn run_wake(s: *const ()) {
        let r = unsafe {
            // can't cast to mutable w/o transmute
            std::mem::transmute::<*const (), &mut Task<F>>(s)
        };
        r.advance();
    }

    fn run_clone(s: *const ()) -> RawWaker {
        RawWaker::new(s, &Task::<F>::VTABLE)
    }

}

