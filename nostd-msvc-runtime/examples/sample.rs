// examples/sample.rs
#![no_std]
#![no_main]

// Import the macro from your crate
use nostd_msvc_runtime::*;

// Use the all-in-one macro for console applications.
// This sets up:
// 1. Global Allocator (HeapAllocator using malloc)
// 2. Panic Handler (calling exit_process)
// 3. Entry Point (mainCRTStartup)
setup_console_app!();

// Required to use Vec, Box, etc. in no_std
use alloc::vec::Vec;

/// The actual logic of your application
fn main() {
    // Test heap allocation using our malloc-based allocator
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // If we had a way to print (like a custom println!), 
    // we could verify the values here.
    
    // The process will exit automatically after this function returns
    // because the macro-generated entry point calls exit_process(0).
}
