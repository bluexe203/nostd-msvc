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

/// Define entry point for Console applications with a custom startup function
#[macro_export]
macro_rules! use_console_entry {
    ($startup_fn:path) => {
        #[no_mangle]
        pub extern "system" fn mainCRTStartup() -> ! {
            $startup_fn(); // Execute the user-specified startup function
            crate::main();
            $crate::exit_process(0);
        }
    };
    // Default fallback to crate's internal startup
    () => {
        $crate::use_console_entry!($crate::startup);
    };
}

/// Define entry point for GUI applications with a custom startup function
#[macro_export]
macro_rules! use_gui_entry {
    ($startup_fn:path) => {
        #[no_mangle]
        pub extern "system" fn WinMainCRTStartup() -> ! {
            $startup_fn(); // Execute the user-specified startup function
            crate::main();
            $crate::exit_process(0);
        }
    };
    // Default fallback to crate's internal startup
    () => {
        $crate::use_gui_entry!($crate::startup);
    };
}

/// Full setup for Console applications (Allows custom startup)
#[macro_export]
macro_rules! setup_console_app {
    ($startup_fn:path) => {
        $crate::use_base_infrastructure!();
        $crate::use_console_entry!($startup_fn);
    };
    () => {
        $crate::setup_console_app!($crate::startup);
    };
}

/// Full setup for GUI applications (Allows custom startup)
#[macro_export]
macro_rules! setup_gui_app {
    ($startup_fn:path) => {
        $crate::use_base_infrastructure!();
        $crate::use_gui_entry!($startup_fn);
    };
    () => {
        $crate::setup_gui_app!($crate::startup);
    };
}

#[macro_export]
macro_rules! define_lib {
    // Overload for "C" (cdecl) ABI: Prefixes function names with "_" on x86
    ($lib_name:expr, "C", { 
        $(
            $(#[$attr:meta])*
            $vis:vis fn $f_name:ident($($arg:ident: $ty:ty),*) $( -> $ret:ty )?;
        )* }) => {
        #[cfg_attr(target_arch = "x86", link(name = $lib_name, kind = "raw-dylib", import_name_type = "undecorated"))]
        #[cfg_attr(not(target_arch = "x86"), link(name = $lib_name, kind = "raw-dylib"))]
        extern "C" {
            $(
                $(#[$attr])*
                #[cfg_attr(
                    target_arch = "x86", 
                    link_name = concat!("_", stringify!($f_name))
                )]
                $vis fn $f_name($($arg: $ty),*) $( -> $ret )?;
            )*
        }
    };

    // Overload for other ABIs (e.g., "system"): Uses original names
    ($lib_name:expr, $abi:literal, { 
        $(
            $(#[$attr:meta])*
            $vis:vis fn $f_name:ident($($arg:ident: $ty:ty),*) $( -> $ret:ty )?;
        )* }) => {
        #[cfg_attr(target_arch = "x86", link(name = $lib_name, kind = "raw-dylib", import_name_type = "undecorated"))]
        #[cfg_attr(not(target_arch = "x86"), link(name = $lib_name, kind = "raw-dylib"))]
        extern $abi {
            $(
                $(#[$attr])*
                $vis fn $f_name($($arg: $ty),*) $( -> $ret )?;
            )*
        }
    };
}