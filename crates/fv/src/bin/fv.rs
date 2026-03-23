// Don't optimize the alloc crate away due to it being otherwise unused.
// https://github.com/rust-lang/rust/issues/64402
#[cfg(feature = "performance-memory-allocator")]
extern crate fv_performance_memory_allocator;

use std::process::ExitCode;

use fv::main as fv_main;

#[allow(unsafe_code)]
fn main() -> ExitCode {
    #[cfg(feature = "performance-memory-allocator")]
    fv_performance_memory_allocator::init();

    // SAFETY: This is safe because we are running it early in `main` before spawning any threads.
    unsafe { fv_main(std::env::args_os()) }
}
