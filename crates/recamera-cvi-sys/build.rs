//! Build script for `recamera-cvi-sys`.
//!
//! Handles link-time discovery of the SG200X SDK libraries. On non-riscv64
//! targets the script is a no-op so the crate compiles on any host.

fn main() {
    // Re-run if the SDK path changes.
    println!("cargo:rerun-if-env-changed=SG200X_SDK_PATH");

    let target = std::env::var("TARGET").unwrap_or_default();

    // Only attempt to link the vendor libraries when cross-compiling for the
    // actual hardware target. This lets macOS / x86_64 builds succeed without
    // the SDK present (the extern functions will never be called there).
    if !target.starts_with("riscv64") {
        return;
    }

    match std::env::var("SG200X_SDK_PATH") {
        Ok(sdk_path) => {
            let lib_dir = format!("{sdk_path}/lib");
            println!("cargo:rustc-link-search=native={lib_dir}");

            // Core CVI multimedia pipeline libraries.
            for lib in &["cvi_mpi", "sys", "vi", "vpss", "venc", "isp"] {
                println!("cargo:rustc-link-lib=dylib={lib}");
            }

            // NPU runtime.
            println!("cargo:rustc-link-lib=dylib=cviruntime");
        }
        Err(_) => {
            println!(
                "cargo:warning=SG200X_SDK_PATH is not set. \
                 Native CVI libraries will not be linked. \
                 Set SG200X_SDK_PATH to the root of your SG200X SDK sysroot \
                 to enable linking."
            );
        }
    }
}
