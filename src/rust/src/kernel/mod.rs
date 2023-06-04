extern crate alloc;

use core::panic::PanicInfo;
use crate::kernel::print::vc_println;

/// Bindings to Nautilus C types, functions, constants, etc., generated by `bindgen`.
#[allow(clippy::all, clippy::undocumented_unsafe_blocks, missing_debug_implementations)]
pub mod bindings;

/// A global allocator that hooks into `kmem_malloc`.
pub mod allocator;

/// Kernel synchronization primitives.
pub mod sync;

/// IRQ handling.
pub mod irq;

/// Character devices.
pub mod chardev;

/// GPU devices.
pub mod gpudev;

/// Shell commands.
pub mod shell;

/// Printing and logging macros.
pub mod print;

/// Error handling.
pub mod error;

/// Timers.
pub mod timer;

/// Threads.
pub mod thread;

/// Cooperative multitasking.
pub mod task;

/// The global panic handler for Rust code, triggering
/// and unrecoverable kernel panic.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    vc_println!("{}", info);
    // SAFETY: FFI call.
    //
    // We don't need to pass the panic message here,
    // since we already printed it above.
    unsafe { bindings::panic(core::ptr::null()); }
}
