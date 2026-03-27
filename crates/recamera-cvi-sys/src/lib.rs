//! FFI bindings for the Sophgo SG2002 CVI vendor libraries.
//!
//! This crate provides low-level, `unsafe` FFI bindings to the CVI multimedia
//! pipeline and neural-network runtime libraries shipped in the SG200X SDK.
//! The bindings cover:
//!
//! - **CVI System** — initialization, pixel format constants, return codes
//! - **VI** (Video Input) — camera sensor capture
//! - **VPSS** (Video Processing Sub-System) — scaling, cropping, color-space conversion
//! - **VENC** (Video Encoder) — H.264/H.265 hardware encoding
//! - **CVI Runtime** — NPU model loading and inference
//!
//! # Current status
//!
//! The bindings in this crate are **placeholders**. They define the type
//! signatures and constants needed for downstream crates to compile on any
//! host (including macOS), but they do **not** yet contain the full generated
//! output from `bindgen`. See [`bindings`] module documentation for
//! regeneration instructions.
//!
//! # Linking
//!
//! On `riscv64gc-unknown-linux-gnu` targets the build script will look for the
//! `SG200X_SDK_PATH` environment variable and emit the appropriate
//! `cargo:rustc-link-lib` directives. On all other targets linking is skipped,
//! allowing the crate to compile (but not run) anywhere.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod bindings;
pub use bindings::*;
