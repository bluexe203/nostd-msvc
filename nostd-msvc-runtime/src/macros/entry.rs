/// Set up base infrastructure (allocator and panic handler)
#[macro_export]
macro_rules! use_base_infrastructure {
    () => {
        extern crate alloc;
        #[global_allocator]
        static ALLOCATOR: $crate::HeapAllocator = $crate::HeapAllocator;

        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            $crate::exit_process(1);
        }
    };
}

/// Define entry point for Console applications with optional startup/cleanup logic
#[macro_export]
macro_rules! use_console_entry {
    ($startup:expr, $cleanup:expr) => {
        #[no_mangle]
        pub extern "system" fn mainCRTStartup() -> ! {
            ($startup)();      // Execute startup (e.g., WSAStartup)
            crate::main();      // Execute user-defined main
            ($cleanup)();      // Execute cleanup (e.g., WSACleanup)
            $crate::exit_process(0);
        }
    };
    // Default: No-op for both startup and cleanup
    () => {
        $crate::use_console_entry!(|| {}, || {});
    };
}

/// Define entry point for GUI applications with optional startup/cleanup logic
#[macro_export]
macro_rules! use_gui_entry {
    ($startup:expr, $cleanup:expr) => {
        #[no_mangle]
        pub extern "system" fn WinMainCRTStartup() -> ! {
            ($startup)();      // Execute startup
            crate::main();      // Execute user-defined main
            ($cleanup)();      // Execute cleanup
            $crate::exit_process(0);
        }
    };
    // Default: No-op for both startup and cleanup
    () => {
        $crate::use_gui_entry!(|| {}, || {});
    };
}

/// Full setup for Console applications (Allows inline closures)
#[macro_export]
macro_rules! setup_console_app {
    ($startup:expr, $cleanup:expr) => {
        $crate::use_base_infrastructure!();
        $crate::use_console_entry!($startup, $cleanup);
    };
    () => {
        $crate::setup_console_app!(|| {}, || {});
    };
}

/// Full setup for GUI applications (Allows inline closures)
#[macro_export]
macro_rules! setup_gui_app {
    ($startup:expr, $cleanup:expr) => {
        $crate::use_base_infrastructure!();
        $crate::use_gui_entry!($startup, $cleanup);
    };
    () => {
        $crate::setup_gui_app!(|| {}, || {});
    };
}