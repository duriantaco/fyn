//! The only purpose of this crate is to pull in `mimalloc` on windows and
//! `tikv-jemallocator` on most other platforms.

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "openbsd"),
    not(target_os = "freebsd"),
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64"
    )
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// Initialize allocator-specific optimizations.
///
/// On platforms using jemalloc, this enables background threads for
/// asynchronous dirty page purging, reducing latency spikes during
/// deallocation-heavy workloads.
pub fn init() {
    #[cfg(all(
        not(target_os = "windows"),
        not(target_os = "openbsd"),
        not(target_os = "freebsd"),
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "powerpc64"
        )
    ))]
    {
        let _ = tikv_jemalloc_ctl::background_thread::write(true);
    }
}
