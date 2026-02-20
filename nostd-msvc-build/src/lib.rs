// SPDX-License-Identifier: 0BSD
pub fn configure_linker() {
    // 1. Tell Cargo NOT to re-run this script unless these specific files change.
    // This is crucial to prevent infinite build loops.
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_GUI");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_XP");

    let target = std::env::var("TARGET").unwrap_or_default();
    let arch = if target.contains("x86_64") { "x86_64" } else { "i686" };

    if arch == "i686" {
        println!("cargo:rustc-link-arg=/SAFESEH:NO");
        
        let aliases = [
            "__alldiv=_alldiv", "_aulldiv=_alldiv", "__aulldiv=_alldiv",
            "__allrem=_allrem", "_aullrem=_allrem", "__aullrem=_allrem",
            "__allshl=_allshl", "__allshr=_allshr",
            "__chkstk=_chkstk",
            "__CxxFrameHandler3=_CxxFrameHandler3",
        ];

        for alias in aliases {
            println!("cargo:rustc-link-arg=/ALTERNATENAME:{}", alias);
        }
    }

    println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmt");
    println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrt");

    // Use environment variables instead of cfg!() to avoid compile-time recursion
    let is_gui = std::env::var("CARGO_FEATURE_GUI").is_ok();
    let is_xp = std::env::var("CARGO_FEATURE_XP").is_ok();

    let (sub, entry) = if is_gui { 
        ("WINDOWS", "WinMainCRTStartup") 
    } else { 
        ("CONSOLE", "mainCRTStartup") 
    };

    let ver = if is_xp {
        if arch == "x86_64" { ",5.02" } else { ",5.01" }
    } else { "" };

    println!("cargo:rustc-link-arg=/SUBSYSTEM:{}{}", sub, ver);
    println!("cargo:rustc-link-arg=/ENTRY:{}", entry);

    println!("cargo:rustc-link-arg=/INCREMENTAL:NO");
    println!("cargo:rustc-link-arg=/OPT:REF");
    println!("cargo:rustc-link-arg=/OPT:ICF");
}
