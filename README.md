# recamera-rs

A Rust SDK for Seeed reCamera -- camera capture, local inference, serial I/O, storage, and system utilities for edge vision applications.

> **Disclaimer:** This is a community project and is not affiliated with or officially maintained by Seeed Studio.

## Usage

Add `recamera` to your `Cargo.toml` with the features you need:

```toml
[dependencies]
recamera = { version = "0.1", features = ["uart", "storage"] }
```

To enable everything:

```toml
[dependencies]
recamera = { version = "0.1", features = ["full"] }
```

## Feature Flags

| Feature   | Description                              | Default |
|-----------|------------------------------------------|---------|
| `camera`  | Camera capture and frame handling        | No      |
| `infer`   | Local inference engine (.cvimodel)       | No      |
| `uart`    | UART / serial communication              | No      |
| `rs485`   | RS-485 helpers (enables `uart`)          | No      |
| `storage` | Image and file storage utilities         | No      |
| `logging` | Logging utilities                        | Yes     |
| `config`  | Configuration loading and validation     | Yes     |
| `system`  | System and device information utilities  | Yes     |
| `full`    | Enables all features                     | No      |

## Crate Structure

| Crate                | Description                                                        |
|----------------------|--------------------------------------------------------------------|
| `recamera`           | Facade crate -- re-exports subcrates based on feature flags        |
| `recamera-core`      | Shared types, errors, and traits                                   |
| `recamera-camera`    | Camera capture and frame handling                                  |
| `recamera-infer`     | Local inference engine for .cvimodel files                         |
| `recamera-cvi-sys`   | Pre-generated FFI bindings for SG2002 CVI libraries                |
| `recamera-uart`      | UART / serial communication                                       |
| `recamera-rs485`     | RS-485 helpers built on top of UART                                |
| `recamera-storage`   | Image and file storage utilities                                   |
| `recamera-logging`   | Logging utilities                                                  |
| `recamera-config`    | Configuration loading and validation                               |
| `recamera-system`    | System and device information utilities                            |

## Status

This project is at an early stage. The API is expected to change as the design stabilizes.

- Pure-Rust crates (`core`, `uart`, `rs485`, `storage`, `logging`, `config`, `system`) are functional.
- FFI crates (`cvi-sys`) include pre-generated bindings for the CVI MPI camera/video libraries (263 functions).
- NPU inference bindings are not yet available (cviruntime headers not included in current SDK release).
- `camera` and `infer` crate implementations are stubbed, pending wiring to the FFI bindings.

## Getting Started

Most users only need to clone this repo and build. The FFI bindings are pre-generated and committed, and the vendor `.so` libraries are already installed on the reCamera device.

### For app developers

Add this SDK as a dependency in your own project. No SDK download required -- the FFI bindings are pre-generated and the vendor `.so` libraries are already on the reCamera device.

```toml
[dependencies]
recamera = { git = "https://github.com/anthropics/recamera-rs", features = ["camera", "uart"] }
```

## How It Works

The vendor C libraries (camera, video, inference) are loaded at **runtime** on the reCamera device using dynamic loading (`dlopen`). No SDK download or compile-time linking is required — just add this crate as a dependency and build.

The FFI bindings in `recamera-cvi-sys` provide:
- Type definitions, structs, enums, and constants (from `bindings.rs`)
- A runtime loader (`CviLibs`) that loads the vendor `.so` libraries on the device and exposes safe Rust wrappers

## Supported Platforms

This SDK can be built on **macOS** or **Linux**. All paths and scripts are portable -- there are no host-specific configurations in the codebase.

## License

Licensed under either of:

- MIT license
- Apache License, Version 2.0

at your option.

## Contributing

Contributions, issues, and suggestions are welcome. Areas where help is especially useful include camera and inference integration, cross-compilation tooling, and documentation.
