use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator {

}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator { }
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!("Allocator::alloc not implemented!");
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unimplemented!("Allocator::dealloc not implemented!");
    }
}
