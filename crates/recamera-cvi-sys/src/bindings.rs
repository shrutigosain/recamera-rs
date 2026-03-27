//! Pre-generated FFI bindings from the SG200X SDK headers.
//!
//! These bindings are **placeholders** that define only the subset of types,
//! constants, and function signatures required by higher-level crates in the
//! `recamera-rs` workspace. The full bindings will be generated from the
//! vendor SDK headers using [`bindgen`](https://rust-lang.github.io/rust-bindgen/).
//!
//! # Regenerating with bindgen
//!
//! ```sh
//! # 1. Install bindgen-cli
//! cargo install bindgen-cli
//!
//! # 2. Point at the SDK sysroot (adjust the path for your setup)
//! export SG200X_SDK_PATH=/path/to/sg200x_sdk
//!
//! # 3. Generate bindings
//! bindgen \
//!     --use-core \
//!     --no-layout-tests \
//!     --allowlist-function "CVI_.*" \
//!     --allowlist-type "CVI_.*" \
//!     --allowlist-var "CVI_.*" \
//!     wrapper.h \
//!     -- \
//!     -I"$SG200X_SDK_PATH/include" \
//!     --target=riscv64-unknown-linux-gnu \
//!     > src/bindings.rs
//! ```
//!
//! After regenerating, add module-level docs back to the top of the file and
//! ensure the crate still compiles on macOS (`cargo check -p recamera-cvi-sys`).

use core::ffi::{c_char, c_int, c_void};

// ---------------------------------------------------------------------------
// CVI System — common return codes and types
// ---------------------------------------------------------------------------

/// Return code indicating success.
pub const CVI_SUCCESS: c_int = 0;

/// Return code indicating a generic failure.
pub const CVI_FAILURE: c_int = -1;

/// Return code indicating an invalid argument.
pub const CVI_ERR_INVALID_ARG: c_int = -2;

/// Opaque handle type used throughout the CVI API.
pub type CVI_VOID = c_void;

/// Signed 32-bit return type used by most CVI functions.
pub type CVI_S32 = c_int;

/// Unsigned 8-bit type (e.g., raw pixel data).
pub type CVI_U8 = u8;

/// Unsigned 32-bit type (e.g., sizes, flags).
pub type CVI_U32 = u32;

/// Boolean type used by the CVI API (0 = false, 1 = true).
pub type CVI_BOOL = c_int;

// ---------------------------------------------------------------------------
// VI (Video Input)
// ---------------------------------------------------------------------------

/// Video Input device identifier.
pub type VI_DEV = CVI_S32;

/// Video Input channel identifier.
pub type VI_CHN = CVI_S32;

/// Video Input pipe identifier.
pub type VI_PIPE = CVI_S32;

extern "C" {
    /// Initialize the VI (Video Input) subsystem.
    ///
    /// # Safety
    ///
    /// Must be called before any other VI function. The caller is responsible
    /// for ensuring the CVI system has been initialized first.
    pub fn CVI_VI_SetDevAttr(ViDev: VI_DEV, pstDevAttr: *const c_void) -> CVI_S32;

    /// Enable a VI device.
    pub fn CVI_VI_EnableDev(ViDev: VI_DEV) -> CVI_S32;

    /// Disable a VI device.
    pub fn CVI_VI_DisableDev(ViDev: VI_DEV) -> CVI_S32;
}

// ---------------------------------------------------------------------------
// VPSS (Video Processing Sub-System)
// ---------------------------------------------------------------------------

/// VPSS group identifier.
pub type VPSS_GRP = CVI_S32;

/// VPSS channel identifier.
pub type VPSS_CHN = CVI_S32;

extern "C" {
    /// Create a VPSS group.
    pub fn CVI_VPSS_CreateGrp(VpssGrp: VPSS_GRP, pstGrpAttr: *const c_void) -> CVI_S32;

    /// Destroy a VPSS group.
    pub fn CVI_VPSS_DestroyGrp(VpssGrp: VPSS_GRP) -> CVI_S32;

    /// Start a VPSS group.
    pub fn CVI_VPSS_StartGrp(VpssGrp: VPSS_GRP) -> CVI_S32;

    /// Stop a VPSS group.
    pub fn CVI_VPSS_StopGrp(VpssGrp: VPSS_GRP) -> CVI_S32;
}

// ---------------------------------------------------------------------------
// VENC (Video Encoder)
// ---------------------------------------------------------------------------

/// VENC channel identifier.
pub type VENC_CHN = CVI_S32;

extern "C" {
    /// Create a VENC channel.
    pub fn CVI_VENC_CreateChn(VeChn: VENC_CHN, pstAttr: *const c_void) -> CVI_S32;

    /// Destroy a VENC channel.
    pub fn CVI_VENC_DestroyChn(VeChn: VENC_CHN) -> CVI_S32;

    /// Start receiving frames on a VENC channel.
    pub fn CVI_VENC_StartRecvFrame(VeChn: VENC_CHN, pstRecvParam: *const c_void) -> CVI_S32;

    /// Stop receiving frames on a VENC channel.
    pub fn CVI_VENC_StopRecvFrame(VeChn: VENC_CHN) -> CVI_S32;
}

// ---------------------------------------------------------------------------
// CVI Runtime — NPU inference
// ---------------------------------------------------------------------------

extern "C" {
    /// Initialize the CVI neural-network runtime.
    ///
    /// # Safety
    ///
    /// Must be called once before loading any model. The `config` pointer may
    /// be null to use default settings.
    pub fn CVI_NN_Init(config: *const c_void) -> CVI_S32;

    /// Shut down the CVI neural-network runtime and release resources.
    pub fn CVI_NN_Deinit() -> CVI_S32;

    /// Load a compiled model (`.cvimodel`) from a file path.
    ///
    /// On success, writes a model handle to `*model`.
    pub fn CVI_NN_LoadModel(model_path: *const c_char, model: *mut *mut c_void) -> CVI_S32;

    /// Unload a previously loaded model.
    pub fn CVI_NN_UnloadModel(model: *mut c_void) -> CVI_S32;

    /// Run inference on the given model.
    ///
    /// Input and output tensors must be set up before calling this function.
    pub fn CVI_NN_Forward(model: *mut c_void) -> CVI_S32;
}
