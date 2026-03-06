use crate::define_lib;
// --- Memory functions (forwarding to msvcrt) ---

define_lib!("msvcrt", "C", {
    pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    pub fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    pub fn strlen(s: *const u8) -> usize;
    pub fn exit(code: i32);
});

// --- Low-level runtime symbols ---
#[no_mangle]
pub unsafe extern "C" fn __CxxFrameHandler3() -> i32 { 0 }
#[used]
static _STUB_CXX: unsafe extern "C" fn() -> i32 = __CxxFrameHandler3;

define_lib!("ntdll", "C", {
    // Arithmetic helpers (x86)
    pub fn _alldiv();
    pub fn _allrem();
    pub fn _aulldiv();
    pub fn _aullrem();
    pub fn _allshl();
    pub fn _allshr();    
});

// Stack probe (Required for large stack allocations)
#[link(name = "ntdll", kind = "raw-dylib")]
extern "C" {
    #[cfg_attr(target_arch = "x86", link_name = "_chkstk")]
    #[cfg_attr(not(target_arch = "x86"), link_name = "__chkstk")]
    pub fn chkstk();
}

// Floating-point flag (Required by MSVC)
#[no_mangle]
pub static _fltused: i32 = 0;

/// Terminate the process with a given exit code.
pub fn exit_process(code: i32) -> ! {
    unsafe {
        exit(code);
    }
    loop {}
}
