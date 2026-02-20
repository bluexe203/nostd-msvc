use core::alloc::{GlobalAlloc, Layout};

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

macro_rules! define_cdecl {
    ($lib_name:expr, { $($f_name:ident($($arg:ident: $ty:ty),*) $( -> $ret:ty )?;)* }) => {
        #[cfg_attr(target_arch = "x86", link(name = $lib_name, kind = "raw-dylib", import_name_type = "undecorated"))]
        #[cfg_attr(not(target_arch = "x86"), link(name = $lib_name, kind = "raw-dylib"))]
        extern "C" {
            $(
                #[cfg_attr(
                    target_arch = "x86", 
                    link_name = concat!("_", stringify!($f_name))
                )]
                pub fn $f_name($($arg: $ty),*) $( -> $ret )?;
            )*
        }
    };
}

define_cdecl!("msvcrt", {
    _aligned_malloc(size: usize, alignment: usize) -> *mut core::ffi::c_void;
    _aligned_free(ptr: *mut core::ffi::c_void);
    _aligned_realloc(ptr: *mut core::ffi::c_void, size: usize, alignment: usize) -> *mut core::ffi::c_void;
});
