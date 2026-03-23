// Don't optimize the alloc crate away due to it being otherwise unused.
// https://github.com/rust-lang/rust/issues/64402
#[cfg(feature = "performance-memory-allocator")]
extern crate fyn_performance_memory_allocator;

use std::process::ExitCode;

use fyn::main as fyn_main;

#[allow(unsafe_code)]
fn main() -> ExitCode {
    #[cfg(feature = "performance-memory-allocator")]
    fyn_performance_memory_allocator::init();

    // SAFETY: This is safe because we are running it early in `main` before spawning any threads.
    unsafe { fyn_main(std::env::args_os()) }
}
