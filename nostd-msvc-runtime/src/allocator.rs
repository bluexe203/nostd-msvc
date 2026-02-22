use core::alloc::{GlobalAlloc, Layout};
use crate::define_lib;

pub struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        _aligned_malloc(layout.size(), layout.align()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        _aligned_free(ptr as *mut _);
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        _aligned_realloc(ptr as *mut _, new_size, layout.align()) as *mut u8
    }
}

define_lib!("msvcrt", "C",{
    pub fn _aligned_malloc(size: usize, alignment: usize) -> *mut core::ffi::c_void;
    pub fn _aligned_free(ptr: *mut core::ffi::c_void);
    pub fn _aligned_realloc(ptr: *mut core::ffi::c_void, size: usize, alignment: usize) -> *mut core::ffi::c_void;
});
