use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::os::raw::c_int;


/// A common utility class for client and server.
/// the contract is the client will only write to
/// the client atomic, and the server only to
/// the server atomic.
pub struct MappedAtomics {
    pub client_write: &'static AtomicU64,
    pub server_write: &'static AtomicU64,
    mmap_ptr: *mut c_void,
}

impl MappedAtomics {

    /// this is only called from the server_loop.
    /// The compilers seem to like this better
    /// in a separate function than in-line by hand.
    #[inline(always)]
    pub fn server_spin_until_change(&self, last_value: u64) -> u64 {
        let mut new_value = last_value;
        while new_value == last_value {
            core::hint::spin_loop();
            new_value = self.client_write.load(Ordering::Relaxed);
        }
        new_value
    }

    pub fn do_server_loop(&self) {
        let mut last_value: u64 = 0;
        loop {
            last_value = self.server_spin_until_change(last_value);
            self.server_write.store(last_value, Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub fn client_run_once(&self, value: u64) {
        self.client_write.store(value, Ordering::Relaxed);

        let mut last_read = !value;

        while value != last_read {
            core::hint::spin_loop();
            last_read = self.server_write.load(Ordering::Relaxed);
        }
    }


    /// open or create the shared memory atomics.
    /// whichever is the first to start should
    /// create, the second should pass 'false' and fail
    /// if the expected named memory doesn't exist.
    pub fn new(do_create: bool) -> MappedAtomics {
        unsafe {
            let mem_fd = MappedAtomics::shm_open(do_create);

            if libc::ftruncate(mem_fd, page_size::get() as i64) < 0 {
                panic!(
                    "can't truncate shared memory FD. error num : {}. page size = {}",
                    *libc::__errno_location(),
                    page_size::get()
                );
            }

            let mem_ptr = MappedAtomics::mmap( mem_fd );

            let first_ptr = mem_ptr as *mut u64;
            // set the second atomic a few cache lines down.
            let second_ptr = (mem_ptr as *mut u8).add(2048) as *mut u64;
            assert_ne!(first_ptr, second_ptr);
            let mapped_atomics = MappedAtomics {
                client_write: &*(first_ptr as *const AtomicU64),
                // scooch down a cache line or two.
                server_write: &*(second_ptr as *const AtomicU64),
                mmap_ptr: mem_ptr,
            };
            // only zero out on creation, lest we romp on the values
            // when the server starts up, after the client has been running.
            if do_create {
                mapped_atomics.client_write.store(0, Ordering::Relaxed);
                mapped_atomics.server_write.store(0, Ordering::Relaxed);
            }

            mapped_atomics
        }
    }
    unsafe fn shm_open(do_create:bool) -> c_int {
        let mem_fd = libc::shm_open(
            crate::SH_MEM_NAME.as_ptr(),
            if do_create {
                libc::O_CREAT | libc::O_RDWR
            } else {
                libc::O_RDWR
            },
            libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP,
        );
        if mem_fd < 0 {
            panic!(
                "can't create shared memory. error num :  {}",
                *libc::__errno_location()
            );
        }
        mem_fd
    }
    unsafe fn mmap(shm_fd:c_int) -> *mut c_void {
        let mem_ptr = libc::mmap(
            std::ptr::null_mut(),
            page_size::get(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            shm_fd,
            0,
        );
        if mem_ptr == libc::MAP_FAILED {
            panic!(
                "mmap shared memory failed. error code : {}",
                *libc::__errno_location()
            );
        }
        mem_ptr
    }

    pub fn close(&self) {
        unsafe {
            libc::munmap(self.mmap_ptr, page_size::get());
            libc::shm_unlink(crate::SH_MEM_NAME.as_ptr());
        }
    }
}
