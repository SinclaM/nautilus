use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

use crate::nk_bindings;

pub struct NkAllocator;

unsafe impl GlobalAlloc for NkAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let malloc_size = layout.pad_to_align().size() as u64;
        // TODO: is kmem_malloc thread-safe?? `NkAllocator` does NOT lock
        let allocated = nk_bindings::kmem_malloc(malloc_size) as *mut u8;
        if allocated as usize % layout.align() != 0 {
            // the current allocator is a buddy allocator,
            // which guarantees this shouldn't happen.
            panic!("kmem_malloc returned unaligned pointer");
        }
        allocated
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        nk_bindings::kmem_free(ptr as *mut c_void);
    }
}

#[global_allocator]
static ALLOCATOR: NkAllocator = NkAllocator {};

#[cfg(not(test))]
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
