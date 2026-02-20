// SPDX-License-Identifier: 0BSD
#![no_std]
// 1. Declare modules
pub mod allocator;
pub mod runtime;
pub mod macros;

// 2. Re-export for the macro ($crate::HeapAllocator)
// Make sure these are EXACTLY here in lib.rs
pub use crate::allocator::HeapAllocator;
pub use crate::runtime::*;
