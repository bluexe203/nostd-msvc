// nostd-msvc-runtime/src/runtime.rs
#![allow(internal_features)]

// --- Memory functions (forwarding to msvcrt) ---

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    extern_msvcrt::memcpy(dest, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    extern_msvcrt::memset(s, c, n)
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    extern_msvcrt::memmove(dest, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    extern_msvcrt::memcmp(s1, s2, n)
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    extern_msvcrt::strlen(s)
}

mod extern_msvcrt {
    #[link(name = "msvcrt", kind = "raw-dylib")]
    extern "C" {
        pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
        pub fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
        pub fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
        pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
        pub fn strlen(s: *const u8) -> usize;
        pub fn exit(code: i32);
    }
}

// --- Low-level runtime symbols ---

/// Stub for C++ exception handling. 
/// Defined in Rust to ensure the linker finds it without needing external DLL resolution for this symbol.
#[no_mangle]
pub extern "C" fn __CxxFrameHandler3() -> i32 {
    0
}

#[link(name = "ntdll", kind = "raw-dylib")]
extern "C" {
    pub fn _alldiv();
    pub fn _allrem();
    pub fn _chkstk();
}

// Floating-point flag (Required by MSVC)
#[no_mangle]
pub static _fltused: i32 = 0;

/// Terminate the process with a given exit code.
pub fn exit_process(code: i32) -> ! {
    unsafe {
        extern_msvcrt::exit(code);
    }
    loop {}
}

/// Perform startup initialization
pub fn startup() {
    // Reserved for future use
}
