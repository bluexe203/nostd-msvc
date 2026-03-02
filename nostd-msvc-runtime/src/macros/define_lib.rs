
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