// SPDX-License-Identifier: 0BSD
pub fn configure_linker() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_GUI");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_XP");

    let target = std::env::var("TARGET").unwrap_or_default();
    let arch = if target.contains("x86_64") { "x86_64" } else { "i686" };

    // raw-dylib uses explicit link_names, 
    // so /ALTERNATENAME aliases for x86 are no longer needed.
    if arch == "i686" {
        println!("cargo:rustc-link-arg=/SAFESEH:NO");
    }

    // Still needed to prevent conflicts with standard libraries
    println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmt");
    println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrt");

    let is_xp = cfg!(feature = "xp");
    let is_gui = cfg!(feature = "gui");

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
    println!("cargo:rustc-link-arg=/OPT:REF");
    println!("cargo:rustc-link-arg=/OPT:ICF");
}