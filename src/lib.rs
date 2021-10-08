#[macro_use]
extern crate lazy_static;

use std::ffi::CStr;
use std::time::Duration;

/// here are general utility functions and some global constants.
/// They are used by both the benchmarking code and the servers
/// so they are in a library

pub mod atomic_spin;
pub mod bench_utils;

pub static SAMPLE_SIZE: usize = 1000;
pub static WARMUP_TIME: Duration = Duration::from_secs(10);
pub static RUN_TIME: Duration = Duration::from_secs(30);
pub static CLIENT_CPU: usize = 4;
pub static SERVER_CPU: &str = "5";

lazy_static! {
    static ref SH_MEM_NAME: &'static CStr =
        unsafe { CStr::from_bytes_with_nul_unchecked("/spinnmem\0".as_bytes()) };
}

