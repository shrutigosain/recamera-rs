//! Build script for `recamera-cvi-sys`.
//!
//! Handles link-time discovery of the reCamera-OS SDK libraries.
//! On non-riscv64 targets the script is a no-op so the crate compiles
//! on any host.
//!
//! Set `SG200X_SDK_PATH` to the extracted SDK root (the directory
//! containing `cvi_mpi/`, `buildroot-2021.05/`, etc.).

fn main() {
    println!("cargo:rerun-if-env-changed=SG200X_SDK_PATH");

    let target = std::env::var("TARGET").unwrap_or_default();

    // Only link when cross-compiling for the actual hardware target.
    if !target.starts_with("riscv64") {
        return;
    }

    match std::env::var("SG200X_SDK_PATH") {
        Ok(sdk_path) => {
            // Libraries are at <SDK_PATH>/cvi_mpi/lib/
            let lib_dir = format!("{sdk_path}/cvi_mpi/lib");
            println!("cargo:rustc-link-search=native={lib_dir}");

            // Core CVI multimedia pipeline libraries.
            for lib in &["sys", "vi", "vpss", "venc", "isp", "ae", "awb"] {
                println!("cargo:rustc-link-lib=dylib={lib}");
            }

            // NPU runtime (available in lib/ even though headers are missing).
            println!("cargo:rustc-link-lib=dylib=cviruntime");
        }
        Err(_) => {
            println!(
                "cargo:warning=SG200X_SDK_PATH is not set. \
                 CVI libraries will not be linked. Set it to the \
                 extracted reCamera-OS SDK root to enable linking."
            );
        }
    }
}
