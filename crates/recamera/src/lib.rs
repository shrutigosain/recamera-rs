//! # recamera
//!
//! Rust SDK for the [Seeed reCamera](https://www.seeedstudio.com/reCamera)
//! family of edge-AI cameras.
//!
//! This crate is a thin facade that re-exports the individual `recamera-*`
//! sub-crates behind Cargo feature flags, so you can pull in only the
//! subsystems you need.
//!
//! ## Quick start
//!
//! Add `recamera` to your `Cargo.toml` with the features you need:
//!
//! ```toml
//! [dependencies]
//! recamera = { version = "0.1", features = ["camera", "uart"] }
//! ```
//!
//! The **default** features are `logging`, `config`, and `system`. Enable
//! `full` to pull in every subsystem:
//!
//! ```toml
//! [dependencies]
//! recamera = { version = "0.1", features = ["full"] }
//! ```
//!
//! ## Feature flags
//!
//! | Feature   | Sub-crate           | Description                              |
//! |-----------|---------------------|------------------------------------------|
//! | `camera`  | `recamera-camera`   | Camera capture (VPSS / VI pipeline)      |
//! | `infer`   | `recamera-infer`    | Neural-network inference (TPU runtime)   |
//! | `uart`    | `recamera-uart`     | UART / serial communication              |
//! | `rs485`   | `recamera-rs485`    | RS-485 over UART helpers                 |
//! | `storage` | `recamera-storage`  | SD card and file-system utilities         |
//! | `logging` | `recamera-logging`  | Structured logging (tracing)             |
//! | `config`  | `recamera-config`   | Configuration loading and validation     |
//! | `system`  | `recamera-system`   | System / device information utilities    |
//! | `full`    | *(all of the above)* | Enable every subsystem                  |

// ---------------------------------------------------------------------------
// Core — always available
// ---------------------------------------------------------------------------

/// Core types, error handling, and traits shared across the SDK.
pub use recamera_core as core;

// Re-export the most commonly used core items at the crate root for
// convenience so users can write `recamera::Error` instead of
// `recamera::core::Error`.
#[doc(inline)]
pub use recamera_core::{Error, FrameData, ImageFormat, Resolution, Result};

// ---------------------------------------------------------------------------
// Feature-gated sub-crate re-exports
// ---------------------------------------------------------------------------

/// Camera capture (requires the **`camera`** feature).
#[cfg(feature = "camera")]
pub use recamera_camera as camera;

/// Neural-network inference (requires the **`infer`** feature).
#[cfg(feature = "infer")]
pub use recamera_infer as infer;

/// UART / serial communication (requires the **`uart`** feature).
#[cfg(feature = "uart")]
pub use recamera_uart as uart;

/// RS-485 over UART helpers (requires the **`rs485`** feature).
#[cfg(feature = "rs485")]
pub use recamera_rs485 as rs485;

/// Storage utilities (requires the **`storage`** feature).
#[cfg(feature = "storage")]
pub use recamera_storage as storage;

/// Structured logging (requires the **`logging`** feature).
#[cfg(feature = "logging")]
pub use recamera_logging as logging;

/// Configuration loading and validation (requires the **`config`** feature).
#[cfg(feature = "config")]
pub use recamera_config as config;

/// System and device utilities (requires the **`system`** feature).
#[cfg(feature = "system")]
pub use recamera_system as system;
